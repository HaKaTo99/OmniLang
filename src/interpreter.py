# OmniLang v0.1 - Interpreter
# Interpreter sederhana untuk AST OmniLang

from parser import Parser
from lexer import tokenize

class Interpreter:
    def __init__(self):
        self.env = {}
    def eval(self, ast):
        for node in ast:
            if node.type == 'VarDecl':
                var_name = node.value
                var_value = int(node.children[0].value)
                self.env[var_name] = var_value
                print(f"{var_name} = {var_value}")

if __name__ == "__main__":
    code = 'let x = 42;'
    tokens = tokenize(code)
    parser = Parser(tokens)
    ast = parser.parse()
    interp = Interpreter()
    interp.eval(ast)
