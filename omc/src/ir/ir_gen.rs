use crate::parser::ast::{Program, Statement, Expression};
use crate::ir::ir_instr::{Instruction, Value, IrOp};
use crate::lexer::token::Token;

pub struct IRGenerator {
    instructions: Vec<Instruction>,
    reg_count: usize,
    label_count: usize,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            reg_count: 0,
            label_count: 0,
        }
    }

    pub fn generate(&mut self, program: &Program) -> Vec<Instruction> {
        for stmt in &program.statements {
            self.gen_statement(stmt);
        }
        self.instructions.clone()
    }

    fn next_reg(&mut self) -> usize {
        self.reg_count += 1;
        self.reg_count
    }

    fn next_label(&mut self, prefix: &str) -> String {
        self.label_count += 1;
        format!("{}_{}", prefix, self.label_count)
    }

    fn emit(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    fn gen_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Let { name, value, .. } => {
                let val = self.gen_expression(value);
                self.emit(Instruction::Store(name.clone(), val));
            }
            Statement::Return(expr) => {
                let val = expr.as_ref().map(|e| self.gen_expression(e));
                self.emit(Instruction::Return(val));
            }
            Statement::Expression(expr) => {
                self.gen_expression(expr);
            }
            Statement::Block(stmts) => {
                for s in stmts {
                    self.gen_statement(s);
                }
            }
            Statement::Function { name, params, body, .. } => {
                let param_names: Vec<String> = params.iter().map(|(n, _)| n.clone()).collect();
                self.emit(Instruction::FunctionStart(name.clone(), param_names));
                self.gen_statement(body);
                self.emit(Instruction::FunctionEnd);
            }
        }
    }

    fn gen_expression(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::Integer(i) => {
                let reg = self.next_reg();
                self.emit(Instruction::Load(reg, Value::LiteralInt(*i)));
                Value::Register(reg)
            }
            Expression::String(s) => {
                let reg = self.next_reg();
                self.emit(Instruction::Load(reg, Value::LiteralString(s.clone())));
                Value::Register(reg)
            }
            Expression::Boolean(b) => {
                let reg = self.next_reg();
                self.emit(Instruction::Load(reg, Value::LiteralBool(*b)));
                Value::Register(reg)
            }
            Expression::Identifier(name) => {
                Value::Variable(name.clone())
            }
            Expression::Infix { left, operator, right } => {
                // Optimization: Constant Folding
                if let (Expression::Integer(l_val), Expression::Integer(r_val)) = (&**left, &**right) {
                    let res = match operator {
                        Token::Plus => Some(l_val + r_val),
                        Token::Minus => Some(l_val - r_val),
                        Token::Star => Some(l_val * r_val),
                        Token::Slash => if *r_val != 0 { Some(l_val / r_val) } else { None },
                        _ => None,
                    };
                    if let Some(val) = res {
                        let reg = self.next_reg();
                        self.emit(Instruction::Load(reg, Value::LiteralInt(val)));
                        return Value::Register(reg);
                    }
                }

                let l_val = self.gen_expression(left);
                let r_val = self.gen_expression(right);
                let reg = self.next_reg();
                let op = match operator {
                    Token::Plus => IrOp::Add,
                    Token::Minus => IrOp::Sub,
                    Token::Star => IrOp::Mul,
                    Token::Slash => IrOp::Div,
                    Token::EqEq => IrOp::Eq,
                    Token::NotEq => IrOp::Ne,
                    Token::Lt => IrOp::Lt,
                    Token::Gt => IrOp::Gt,
                    Token::LtEq => IrOp::Le,
                    Token::GtEq => IrOp::Ge,
                    _ => panic!("Unknown operator in IR generation: {:?}", operator),
                };
                self.emit(Instruction::Binary(reg, op, l_val, r_val));
                Value::Register(reg)
            }
            Expression::Prefix { operator, right } => {
                let r_val = self.gen_expression(right);
                let reg = self.next_reg();
                // Prefix -x is implemented as 0 - x for now in IR
                if let Token::Minus = operator {
                    self.emit(Instruction::Binary(reg, IrOp::Sub, Value::LiteralInt(0), r_val));
                }
                Value::Register(reg)
            }
            Expression::If { condition, consequence, alternative } => {
                let cond_val = self.gen_expression(condition);
                let true_label = self.next_label("if_true");
                let false_label = self.next_label("if_false");
                let end_label = self.next_label("if_end");

                self.emit(Instruction::CondJump(cond_val, true_label.clone(), false_label.clone()));

                self.emit(Instruction::Label(true_label));
                self.gen_statement(consequence);
                self.emit(Instruction::Jump(end_label.clone()));

                self.emit(Instruction::Label(false_label));
                if let Some(alt) = alternative {
                    self.gen_statement(alt);
                }
                self.emit(Instruction::Jump(end_label.clone()));

                self.emit(Instruction::Label(end_label));
                Value::LiteralBool(true) // If expressions need a return value if they are expressions
            }
            Expression::Call { function, arguments } => {
                let func_name = if let Expression::Identifier(n) = &**function {
                    n.clone()
                } else {
                    "anonymous_func".to_string()
                };
                let mut args = vec![];
                for arg in arguments {
                    args.push(self.gen_expression(arg));
                }
                let reg = self.next_reg();
                self.emit(Instruction::Call(reg, func_name, args));
                Value::Register(reg)
            }
            Expression::Match(match_expr) => {
                // 1. Evaluate Scrutinee
                let val_reg = self.gen_expression(&*match_expr.scrutinee);
                
                let match_end_label = self.next_label("match_end");
                let mut next_check_label = self.next_label("arm_check");

                for (i, arm) in match_expr.arms.iter().enumerate() {
                    let current_check_label = next_check_label;
                    next_check_label = self.next_label(&format!("arm_check_{}", i+1));
                    let body_label = self.next_label(&format!("arm_body_{}", i));

                    // Emit label for this arm's check (except first one which follows immediately? 
                    // No, simpler to just jump to first check or fallthrough.
                    // Let's emit Label(current_check_label) if it's not the very first start?
                    // actually, we fall through from scrutinee evaluation to first check.
                    // So we don't strictly need a label for the first check unless we jump to it.
                    // But to be consistent, we can label it.
                    if i > 0 {
                        self.emit(Instruction::Label(current_check_label.clone()));
                    }

                    match &arm.pattern {
                        crate::parser::ast::Pattern::Literal(lit_expr) => {
                            // Evaluate literal to a value (optimization: literal should be constant)
                            // But our gen_expression handles literals by loading them.
                            let pat_val_reg = self.gen_expression(lit_expr);
                            
                            // Emit Check
                            let cmp_reg = self.next_reg();
                            self.emit(Instruction::Binary(cmp_reg, IrOp::Eq, val_reg.clone(), pat_val_reg));
                            
                            self.emit(Instruction::CondJump(Value::Register(cmp_reg), body_label.clone(), next_check_label.clone()));
                        }
                        crate::parser::ast::Pattern::Wildcard | crate::parser::ast::Pattern::Identifier(_) => {
                            // Wildcard/Ident matches everything. 
                            // Treat Identifier as wildcard for now (binding not implemented yet)
                            self.emit(Instruction::Jump(body_label.clone()));
                        }
                    }

                    // Body
                    self.emit(Instruction::Label(body_label));
                    self.gen_statement(&arm.body);
                    self.emit(Instruction::Jump(match_end_label.clone()));
                }

                // If no arms match (and no wildcard), we fall through to next_check_label.
                // We should probably have a default panic or just end?
                // Rust requires exhaustive match. For now, just label it and continue.
                self.emit(Instruction::Label(next_check_label));
                
                self.emit(Instruction::Label(match_end_label));
                
                // Match is an expression, should return a value.
                // For now, assume it returns 0/void if we don't have block-returns implemented properly in all cases
                // Or if the body was a statement block that didn't yield a register.
                // TODO: Fix expression-block return values in IR
                Value::LiteralInt(0) 
            }
        }
    }
}
