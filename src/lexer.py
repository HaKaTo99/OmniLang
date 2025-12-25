# OmniLang v0.1 - Lexer
# Tokenizer sederhana untuk subset sintaks OmniLang

import re

TOKEN_SPEC = [
    ('NUMBER',   r'\d+'),
    ('ID',       r'[A-Za-z_][A-Za-z0-9_]*'),
    ('STRING',   r'".*?"'),
    ('OP',       r'[+\-*/=<>!&|]'),
    ('LBRACE',   r'\{'),
    ('RBRACE',   r'\}'),
    ('LPAREN',   r'\('),
    ('RPAREN',   r'\)'),
    ('COLON',    r':'),
    ('COMMA',    r','),
    ('SEMICOLON',r';'),
    ('SKIP',     r'[ \t\n]+'),
    ('MISMATCH', r'.'),
]

TOKEN_REGEX = '|'.join('(?P<%s>%s)' % pair for pair in TOKEN_SPEC)

class Token:
    def __init__(self, type_, value):
        self.type = type_
        self.value = value
    def __repr__(self):
        return f"Token({self.type}, {self.value})"

def tokenize(code):
    for mo in re.finditer(TOKEN_REGEX, code):
        kind = mo.lastgroup
        value = mo.group()
        if kind == 'SKIP':
            continue
        elif kind == 'MISMATCH':
            raise RuntimeError(f'Unexpected character: {value}')
        yield Token(kind, value)

if __name__ == "__main__":
    code = 'let x = 42;'
    print(list(tokenize(code)))
