use omnilang_core::lexer::Lexer;
use omnilang_core::parser::Parser;

#[test]
fn parser_handles_units_and_nested_loops() {
    let source = include_str!("../examples/nested_loops_units.omni");
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("lexer should succeed");
    let mut parser = Parser::new(tokens);
    let policy = parser.parse_policy().expect("parser should succeed");
    assert!(policy.rules.len() >= 3, "expected multiple rules parsed");
}
