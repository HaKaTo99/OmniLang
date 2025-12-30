use omnilang_core::lexer::Lexer;
use omnilang_core::parser::Parser;

fn parse_text(src: &str) -> Result<omnilang_core::ast::Policy, String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_policy()
}

#[test]
fn headers_are_case_insensitive_and_tradeoff_is_accepted() {
    let src = r#"
intent: Sample intent
actor:
- primary: Operator
context:
- domain: Test
rule:
- if BatteryLevel < 20% then ReturnToHome
impact:
- trade-off: latency vs accuracy
"#;

    let policy = parse_text(src).expect("parser should handle lowercase headers and trade-off");
    assert_eq!(policy.intent.as_deref(), Some("Sample intent"));
    assert_eq!(policy.actors.len(), 1);
    assert_eq!(policy.rules.len(), 1);
    assert_eq!(policy.impacts.len(), 1);
    assert_eq!(policy.impacts[0].kind, "TradeOff");
}
