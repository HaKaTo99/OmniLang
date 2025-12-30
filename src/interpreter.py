# Deprecated v0.1 interpreter shim.
# For v1.0, use omnilang.Parser and policy runtime instead.

from omnilang import Lexer, Parser  # type: ignore


def parse(code: str):
    lexer = Lexer(code)
    tokens = lexer.tokenize()
    parser = Parser(tokens)
    return parser.parse()


if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Usage: python interpreter.py <file.omni>")
        sys.exit(1)

    with open(sys.argv[1], "r", encoding="utf-8") as f:
        policy = parse(f.read())
        print(policy)
