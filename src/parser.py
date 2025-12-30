"""Deprecated v0.1 parser shim. Delegates to omnilang.Parser (v1.0)."""

from omnilang import Lexer, Parser as OmniParser  # type: ignore


class ASTNode:
    """Kept for backwards compatibility with any legacy consumers."""

    def __init__(self, type_, value=None, children=None):
        self.type = type_
        self.value = value
        self.children = children or []


def parse(code: str):
    lexer = Lexer(code)
    tokens = lexer.tokenize()
    parser = OmniParser(tokens)
    return parser.parse()


if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Usage: python parser.py <file.omni>")
        sys.exit(1)

    with open(sys.argv[1], "r", encoding="utf-8") as f:
        policy = parse(f.read())
        print(policy)
