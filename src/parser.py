# OmniLang v0.1 - Parser
# Parser sederhana untuk AST dari token OmniLang

from lexer import tokenize, Token

class ASTNode:
    def __init__(self, type_, value=None, children=None):
        self.type = type_
        self.value = value
        self.children = children or []
    def __repr__(self):
        return f"ASTNode({self.type}, {self.value}, {self.children})"

class Parser:
    def __init__(self, tokens):
        self.tokens = list(tokens)
        self.pos = 0
    def peek(self):
        return self.tokens[self.pos] if self.pos < len(self.tokens) else None
    def advance(self):
        self.pos += 1
    def match(self, type_):
        tok = self.peek()
        if tok and tok.type == type_:
            self.advance()
            return tok
        return None
    def parse_var_decl(self):
        # let x = 42;
        if self.match('ID') and self.peek().value == 'let':
            self.advance()
            id_tok = self.match('ID')
            self.match('OP') # =
            expr = self.match('NUMBER')
            self.match('SEMICOLON')
            return ASTNode('VarDecl', id_tok.value, [ASTNode('Number', expr.value)])
        return None
    def parse(self):
        nodes = []
        while self.pos < len(self.tokens):
            node = self.parse_var_decl()
            if node:
                nodes.append(node)
            else:
                self.advance()
        return nodes

if __name__ == "__main__":
    code = 'let x = 42;'
    tokens = tokenize(code)
    parser = Parser(tokens)
    ast = parser.parse()
    print(ast)
