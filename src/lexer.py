"""Deprecated v0.1 lexer shim. Use omnilang.Lexer instead."""

from omnilang import Lexer  # type: ignore


def tokenize(code):
    lexer = Lexer(code)
    return lexer.tokenize()


if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Usage: python lexer.py <file.omni>")
        sys.exit(1)

    with open(sys.argv[1], "r", encoding="utf-8") as f:
        print(list(tokenize(f.read())))
