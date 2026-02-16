use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    LiteralInt(i64),
    LiteralString(String),
    LiteralBool(bool),
    Register(usize),
    Variable(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::LiteralInt(i) => write!(f, "{}", i),
            Value::LiteralString(s) => write!(f, "\"{}\"", s),
            Value::LiteralBool(b) => write!(f, "{}", b),
            Value::Register(r) => write!(f, "%{}", r),
            Value::Variable(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrOp {
    Add, Sub, Mul, Div,
    Eq, Ne, Lt, Gt, Le, Ge,
}

impl fmt::Display for IrOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self {
            IrOp::Add => "add",
            IrOp::Sub => "sub",
            IrOp::Mul => "mul",
            IrOp::Div => "div",
            IrOp::Eq => "eq",
            IrOp::Ne => "ne",
            IrOp::Lt => "lt",
            IrOp::Gt => "gt",
            IrOp::Le => "le",
            IrOp::Ge => "ge",
        };
        write!(f, "{}", op)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Load(usize, Value),           // %dest = load value
    Binary(usize, IrOp, Value, Value), // %dest = op v1, v2
    Store(String, Value),         // store var, value
    Label(String),                // label:
    Jump(String),                 // jmp label
    CondJump(Value, String, String), // cjmp val, true_label, false_label
    Call(usize, String, Vec<Value>), // %dest = call func(args)
    Return(Option<Value>),        // ret val
    FunctionStart(String, Vec<String>), // fn name(params)
    FunctionEnd,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Load(dest, val) => write!(f, "  %{} = load {}", dest, val),
            Instruction::Binary(dest, op, v1, v2) => write!(f, "  %{} = {} {}, {}", dest, op, v1, v2),
            Instruction::Store(var, val) => write!(f, "  store {}, {}", var, val),
            Instruction::Label(name) => write!(f, "{}:", name),
            Instruction::Jump(target) => write!(f, "  jmp {}", target),
            Instruction::CondJump(val, t, f_exp) => write!(f, "  cjmp {}, {}, {}", val, t, f_exp),
            Instruction::Call(dest, func, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "  %{} = call {}({})", dest, func, args_str.join(", "))
            },
            Instruction::Return(val) => match val {
                Some(v) => write!(f, "  ret {}", v),
                None => write!(f, "  ret"),
            },
            Instruction::FunctionStart(name, params) => {
                write!(f, "fn {}({}) {{", name, params.join(", "))
            },
            Instruction::FunctionEnd => write!(f, "}}"),
        }
    }
}
