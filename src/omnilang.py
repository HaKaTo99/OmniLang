"""
OmniLang v0.1 Interpreter (Prototype)
Author: Herman Krisnanto
Description: Universal programming language interpreter with ownership, GC, and multi-platform support
"""
import re
import sys
from typing import List, Any, Dict, Optional

# Token types (expanded for v0.1 features)
TOKEN_TYPES = [
    ('COMMENT', r'//.*'),
    ('NUMBER', r'\d+(\.\d+)?'),
    ('STRING', r'"[^"]*"'),
    ('TRUE', r'true'),
    ('FALSE', r'false'),
    ('MOD', r'mod'),
    ('STRUCT', r'struct'),
    ('FUNC', r'func'),
    ('LET', r'let'),
    ('IF', r'if'),
    ('ELSE', r'else'),
    ('RETURN', r'return'),
    ('IDENT', r'[A-Za-z_][A-Za-z0-9_]*'),
    ('ARROW', r'->'),
    ('LBRACE', r'\{'),
    ('RBRACE', r'\}'),
    ('LPAREN', r'\('),
    ('RPAREN', r'\)'),
    ('COLON', r':'),
    ('COMMA', r','),
    ('SEMICOLON', r';'),
    ('AMPERSAND', r'&'),
    ('DOT', r'\.'),
    ('PLUS', r'\+'),
    ('MINUS', r'-'),
    ('MUL', r'\*'),
    ('DIV', r'/'),
    ('EQEQ', r'=='),
    ('NEQ', r'!='),
    ('LTE', r'<='),
    ('GTE', r'>='),
    ('LT', r'<'),
    ('GT', r'>'),
    ('AND', r'&&'),
    ('OR', r'\|\|'),
    ('NOT', r'!'),
    ('EQ', r'='),
    ('WS', r'\s+'),
]

class Token:
    def __init__(self, type_, value):
        self.type = type_
        self.value = value
    def __repr__(self):
        return f"Token({self.type}, {self.value})"

class Lexer:
    def __init__(self, code):
        self.code = code
        self.tokens = []
        
    def tokenize(self):
        pos = 0
        while pos < len(self.code):
            match = None
            for type_, regex in TOKEN_TYPES:
                pattern = re.compile(regex)
                match = pattern.match(self.code, pos)
                if match:
                    if type_ not in ('WS', 'COMMENT'):  # Skip whitespace and comments
                        self.tokens.append(Token(type_, match.group(0)))
                    pos = match.end(0)
                    break
            if not match:
                raise SyntaxError(f"Illegal character at position {pos}: {self.code[pos]}")
        return self.tokens

# AST Node classes
class ASTNode:
    pass

class Module(ASTNode):
    def __init__(self, name, body):
        self.name = name
        self.body = body  # List of Struct or Function declarations

class Struct(ASTNode):
    def __init__(self, name, fields):
        self.name = name
        self.fields = fields  # Dict of {field_name: type}

class Function(ASTNode):
    def __init__(self, name, params, return_type, body):
        self.name = name
        self.params = params  # List of (param_name, type)
        self.return_type = return_type
        self.body = body  # List of statements

class VarDecl(ASTNode):
    def __init__(self, name, type_, value):
        self.name = name
        self.type = type_
        self.value = value

class IfStmt(ASTNode):
    def __init__(self, condition, then_block, else_block=None):
        self.condition = condition
        self.then_block = then_block
        self.else_block = else_block

class ReturnStmt(ASTNode):
    def __init__(self, value=None):
        self.value = value

class ExprStmt(ASTNode):
    def __init__(self, expr):
        self.expr = expr

class BinaryOp(ASTNode):
    def __init__(self, left, op, right):
        self.left = left
        self.op = op
        self.right = right

class UnaryOp(ASTNode):
    def __init__(self, op, operand):
        self.op = op
        self.operand = operand

class FunctionCall(ASTNode):
    def __init__(self, name, args):
        self.name = name
        self.args = args

class MemberAccess(ASTNode):
    def __init__(self, obj, member):
        self.obj = obj
        self.member = member

class Identifier(ASTNode):
    def __init__(self, name):
        self.name = name

class Literal(ASTNode):
    def __init__(self, value, type_):
        self.value = value
        self.type = type_

class StructLiteral(ASTNode):
    def __init__(self, struct_name, fields):
        self.struct_name = struct_name
        self.fields = fields  # Dict of {field_name: expr}

class BorrowExpr(ASTNode):
    def __init__(self, expr):
        self.expr = expr

# Enhanced Parser for OmniLang v0.1
class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0
        
    def parse(self):
        """Parse module declaration"""
        self.consume('MOD')
        mod_name = self.consume('IDENT').value
        self.consume('LBRACE')
        body = []
        while not self.match('RBRACE'):
            if self.match('STRUCT'):
                body.append(self.parse_struct())
            elif self.match('FUNC'):
                body.append(self.parse_function())
            else:
                raise SyntaxError(f"Unexpected token: {self.current()}")
        self.consume('RBRACE')
        return Module(mod_name, body)
    
    def parse_struct(self):
        """Parse struct declaration"""
        self.consume('STRUCT')
        name = self.consume('IDENT').value
        self.consume('LBRACE')
        fields = {}
        while not self.match('RBRACE'):
            field_name = self.consume('IDENT').value
            self.consume('COLON')
            field_type = self.parse_type()
            fields[field_name] = field_type
            if self.match('COMMA'):
                self.consume('COMMA')
        self.consume('RBRACE')
        return Struct(name, fields)
    
    def parse_function(self):
        """Parse function declaration"""
        self.consume('FUNC')
        name = self.consume('IDENT').value
        self.consume('LPAREN')
        params = []
        while not self.match('RPAREN'):
            param_name = self.consume('IDENT').value
            self.consume('COLON')
            param_type = self.parse_type()
            params.append((param_name, param_type))
            if self.match('COMMA'):
                self.consume('COMMA')
        self.consume('RPAREN')
        
        return_type = None
        if self.match('ARROW'):
            self.consume('ARROW')
            return_type = self.parse_type()
        
        body = self.parse_block()
        return Function(name, params, return_type, body)
    
    def parse_type(self):
        """Parse type annotation"""
        if self.match('AMPERSAND'):
            self.consume('AMPERSAND')
            return ('borrow', self.parse_type())
        elif self.match('IDENT'):
            return self.consume('IDENT').value
        else:
            raise SyntaxError(f"Expected type, got {self.current()}")
    
    def parse_block(self):
        """Parse block of statements"""
        self.consume('LBRACE')
        statements = []
        while not self.match('RBRACE'):
            statements.append(self.parse_statement())
        self.consume('RBRACE')
        return statements
    
    def parse_statement(self):
        """Parse statement"""
        if self.match('LET'):
            return self.parse_var_decl()
        elif self.match('IF'):
            return self.parse_if_stmt()
        elif self.match('RETURN'):
            return self.parse_return_stmt()
        else:
            return self.parse_expr_stmt()
    
    def parse_var_decl(self):
        """Parse variable declaration"""
        self.consume('LET')
        name = self.consume('IDENT').value
        type_ = None
        if self.match('COLON'):
            self.consume('COLON')
            type_ = self.parse_type()
        self.consume('EQ')
        value = self.parse_expr()
        self.consume('SEMICOLON')
        return VarDecl(name, type_, value)
    
    def parse_if_stmt(self):
        """Parse if statement"""
        self.consume('IF')
        condition = self.parse_expr()
        then_block = self.parse_block()
        else_block = None
        if self.match('ELSE'):
            self.consume('ELSE')
            else_block = self.parse_block()
        return IfStmt(condition, then_block, else_block)
    
    def parse_return_stmt(self):
        """Parse return statement"""
        self.consume('RETURN')
        value = None
        if not self.match('SEMICOLON'):
            value = self.parse_expr()
        self.consume('SEMICOLON')
        return ReturnStmt(value)
    
    def parse_expr_stmt(self):
        """Parse expression statement"""
        expr = self.parse_expr()
        self.consume('SEMICOLON')
        return ExprStmt(expr)
    
    def parse_expr(self):
        """Parse expression (operator precedence)"""
        return self.parse_or()
    
    def parse_or(self):
        left = self.parse_and()
        while self.match('OR'):
            op = self.consume('OR').value
            right = self.parse_and()
            left = BinaryOp(left, op, right)
        return left
    
    def parse_and(self):
        left = self.parse_equality()
        while self.match('AND'):
            op = self.consume('AND').value
            right = self.parse_equality()
            left = BinaryOp(left, op, right)
        return left
    
    def parse_equality(self):
        left = self.parse_comparison()
        while self.match('EQEQ') or self.match('NEQ'):
            op = self.current().value
            self.pos += 1
            right = self.parse_comparison()
            left = BinaryOp(left, op, right)
        return left
    
    def parse_comparison(self):
        left = self.parse_term()
        while self.match('LT') or self.match('GT') or self.match('LTE') or self.match('GTE'):
            op = self.current().value
            self.pos += 1
            right = self.parse_term()
            left = BinaryOp(left, op, right)
        return left
    
    def parse_term(self):
        left = self.parse_factor()
        while self.match('PLUS') or self.match('MINUS'):
            op = self.current().value
            self.pos += 1
            right = self.parse_factor()
            left = BinaryOp(left, op, right)
        return left
    
    def parse_factor(self):
        left = self.parse_unary()
        while self.match('MUL') or self.match('DIV'):
            op = self.current().value
            self.pos += 1
            right = self.parse_unary()
            left = BinaryOp(left, op, right)
        return left
    
    def parse_unary(self):
        if self.match('NOT') or self.match('MINUS'):
            op = self.current().value
            self.pos += 1
            return UnaryOp(op, self.parse_unary())
        elif self.match('AMPERSAND'):
            self.consume('AMPERSAND')
            return BorrowExpr(self.parse_call())
        return self.parse_call()
    
    def parse_call(self):
        expr = self.parse_primary()
        while True:
            if self.match('LPAREN'):
                self.consume('LPAREN')
                args = []
                while not self.match('RPAREN'):
                    args.append(self.parse_expr())
                    if self.match('COMMA'):
                        self.consume('COMMA')
                self.consume('RPAREN')
                expr = FunctionCall(expr.name if isinstance(expr, Identifier) else expr, args)
            elif self.match('DOT'):
                self.consume('DOT')
                member = self.consume('IDENT').value
                expr = MemberAccess(expr, member)
            else:
                break
        return expr
    
    def parse_primary(self):
        """Parse primary expression"""
        if self.match('NUMBER'):
            tok = self.consume('NUMBER')
            value = float(tok.value) if '.' in tok.value else int(tok.value)
            return Literal(value, 'Float' if '.' in tok.value else 'Int')
        elif self.match('STRING'):
            tok = self.consume('STRING')
            return Literal(tok.value.strip('"'), 'String')
        elif self.match('TRUE'):
            self.consume('TRUE')
            return Literal(True, 'Bool')
        elif self.match('FALSE'):
            self.consume('FALSE')
            return Literal(False, 'Bool')
        elif self.match('IDENT'):
            name = self.consume('IDENT').value
            if self.match('LBRACE'):
                # Struct literal
                self.consume('LBRACE')
                fields = {}
                while not self.match('RBRACE'):
                    field_name = self.consume('IDENT').value
                    self.consume('COLON')
                    field_value = self.parse_expr()
                    fields[field_name] = field_value
                    if self.match('COMMA'):
                        self.consume('COMMA')
                self.consume('RBRACE')
                return StructLiteral(name, fields)
            return Identifier(name)
        elif self.match('LPAREN'):
            self.consume('LPAREN')
            expr = self.parse_expr()
            self.consume('RPAREN')
            return expr
        else:
            raise SyntaxError(f"Unexpected token: {self.current()}")
    
    def current(self):
        if self.pos >= len(self.tokens):
            raise SyntaxError("Unexpected end of file")
        return self.tokens[self.pos]
    
    def match(self, type_):
        return self.pos < len(self.tokens) and self.tokens[self.pos].type == type_
    
    def consume(self, type_):
        if not self.match(type_):
            raise SyntaxError(f"Expected {type_}, got {self.current().type if self.pos < len(self.tokens) else 'EOF'}")
        tok = self.tokens[self.pos]
        self.pos += 1
        return tok

# Interpreter / Evaluator
class Interpreter:
    def __init__(self):
        self.globals = {}
        self.locals = {}
        self.structs = {}
        self.functions = {}
        
    def eval_module(self, module: Module):
        """Evaluate module"""
        for item in module.body:
            if isinstance(item, Struct):
                self.structs[item.name] = item
            elif isinstance(item, Function):
                self.functions[item.name] = item
        
        # Execute main function if exists
        if 'main' in self.functions:
            self.eval_function_call('main', [])
    
    def eval_function_call(self, name: str, args: List[Any]):
        """Evaluate function call"""
        if name == 'println':
            # Built-in function
            print(args[0] if args else '')
            return None
        
        if name not in self.functions:
            raise NameError(f"Function '{name}' not defined")
        
        func = self.functions[name]
        
        # Create new scope
        old_locals = self.locals
        self.locals = {}
        
        # Bind parameters
        for (param_name, _), arg in zip(func.params, args):
            self.locals[param_name] = arg
        
        # Execute function body
        result = None
        try:
            for stmt in func.body:
                result = self.eval_statement(stmt)
                if isinstance(stmt, ReturnStmt):
                    break
        finally:
            self.locals = old_locals
        
        return result
    
    def eval_statement(self, stmt: ASTNode):
        """Evaluate statement"""
        if isinstance(stmt, VarDecl):
            value = self.eval_expr(stmt.value)
            self.locals[stmt.name] = value
            return None
        elif isinstance(stmt, IfStmt):
            condition = self.eval_expr(stmt.condition)
            if condition:
                for s in stmt.then_block:
                    self.eval_statement(s)
            elif stmt.else_block:
                for s in stmt.else_block:
                    self.eval_statement(s)
            return None
        elif isinstance(stmt, ReturnStmt):
            return self.eval_expr(stmt.value) if stmt.value else None
        elif isinstance(stmt, ExprStmt):
            return self.eval_expr(stmt.expr)
        else:
            raise NotImplementedError(f"Statement type {type(stmt)} not implemented")
    
    def eval_expr(self, expr: ASTNode):
        """Evaluate expression"""
        if isinstance(expr, Literal):
            return expr.value
        elif isinstance(expr, Identifier):
            if expr.name in self.locals:
                return self.locals[expr.name]
            elif expr.name in self.globals:
                return self.globals[expr.name]
            else:
                raise NameError(f"Variable '{expr.name}' not defined")
        elif isinstance(expr, BinaryOp):
            left = self.eval_expr(expr.left)
            right = self.eval_expr(expr.right)
            if expr.op == '+':
                return left + right
            elif expr.op == '-':
                return left - right
            elif expr.op == '*':
                return left * right
            elif expr.op == '/':
                return left / right
            elif expr.op == '==':
                return left == right
            elif expr.op == '!=':
                return left != right
            elif expr.op == '<':
                return left < right
            elif expr.op == '>':
                return left > right
            elif expr.op == '<=':
                return left <= right
            elif expr.op == '>=':
                return left >= right
            elif expr.op == '&&':
                return left and right
            elif expr.op == '||':
                return left or right
            else:
                raise NotImplementedError(f"Binary operator {expr.op} not implemented")
        elif isinstance(expr, UnaryOp):
            operand = self.eval_expr(expr.operand)
            if expr.op == '-':
                return -operand
            elif expr.op == '!':
                return not operand
            else:
                raise NotImplementedError(f"Unary operator {expr.op} not implemented")
        elif isinstance(expr, FunctionCall):
            func_name = expr.name if isinstance(expr.name, str) else expr.name.name
            args = [self.eval_expr(arg) for arg in expr.args]
            return self.eval_function_call(func_name, args)
        elif isinstance(expr, StructLiteral):
            if expr.struct_name not in self.structs:
                raise NameError(f"Struct '{expr.struct_name}' not defined")
            values = {k: self.eval_expr(v) for k, v in expr.fields.items()}
            return {'__struct__': expr.struct_name, **values}
        elif isinstance(expr, MemberAccess):
            obj = self.eval_expr(expr.obj)
            if not isinstance(obj, dict) or '__struct__' not in obj:
                raise TypeError("Member access on non-struct")
            if expr.member not in obj:
                raise AttributeError(f"Struct has no member '{expr.member}'")
            return obj[expr.member]
        elif isinstance(expr, BorrowExpr):
            # For now, just evaluate the inner expression
            # Full borrow checking would be done at compile time
            return self.eval_expr(expr.expr)
        else:
            raise NotImplementedError(f"Expression type {type(expr)} not implemented")

def main():
    if len(sys.argv) < 2:
        print("Usage: python omnilang.py <source.omni>")
        print("Example: python src/omnilang.py examples/hello.omni")
        return
    
    try:
        with open(sys.argv[1], 'r', encoding='utf-8') as f:
            code = f.read()
        
        # Lexical analysis
        lexer = Lexer(code)
        tokens = lexer.tokenize()
        
        # Parsing
        parser = Parser(tokens)
        ast = parser.parse()
        
        # Interpretation
        interpreter = Interpreter()
        interpreter.eval_module(ast)
        
    except FileNotFoundError:
        print(f"Error: File '{sys.argv[1]}' not found")
    except SyntaxError as e:
        print(f"Syntax Error: {e}")
    except Exception as e:
        print(f"Runtime Error: {e}")

if __name__ == '__main__':
    main()
