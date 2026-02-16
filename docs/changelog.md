# OmniLang Changelog

## v0.2.1-alpha (2026-02-16)
**Focus**: Parser Grammar & Control Flow

### Features
- **Pratt Parser**: Full expression parsing with precedence (Prefix, Infix, Grouping).
- **Control Flow**:
    - `if` expressions (`if x > 10 { ... } else { ... }`).
    - Block statements (`{ ... }`).
- **Functions**:
    - Parsing function definitions (`fn name(params) { ... }`).
    - Parsing function calls (`add(x, y)`).
- **Lexer Fixes**: 
    - Fixed specific separator consumption bugs.
    - Added support for comparison operators (`<`, `>`, `!=`, etc.).

### Known Issues
- Minor parsing error with braces in complex nested statements under investigation (suspected `expect_peek` interaction).

## v0.2.0-alpha (Basic Parser)
- Initial AST and Lexer.

## Next Steps (v0.3)
- Semantic Analysis (Symbol Table).
- Type Checking foundation.
