use omnilang_core::lexer::Lexer;
use omnilang_core::parser::Parser;

fn parse_text(src: &str) -> Result<omnilang_core::ast::Policy, String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_policy()
}

#[test]
fn rule_without_then_fails() {
    let src = r#"
INTENT: Test
RULE:
- IF BatteryLevel < 20 ReturnToHome
"#;
    let err = parse_text(src).unwrap_err();
    assert!(err.to_lowercase().contains("unexpected"));
}

#[test]
fn unknown_section_fails() {
    let src = r#"
INTENT: Test
UNKNOWN:
- something
"#;
    let err = parse_text(src).unwrap_err();
    assert!(err.to_lowercase().contains("unexpected"));
}
