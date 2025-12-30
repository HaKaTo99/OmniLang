use omnilang_core::{lexer::Lexer, parser::Parser, runtime::Runtime};

fn parse_policy(source: &str) -> omnilang_core::ast::Policy {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("lexing failed");
    let mut parser = Parser::new(tokens);
    parser.parse_policy().expect("parse failed")
}

#[test]
fn for_loop_binds_iterator_value() {
    let omni = r#"
INTENT: Test iterator binding
RULE:
- FOR item IN items {
    - IF item == 1 THEN ActOne
    - IF item == 2 THEN ActTwo
}
"#;

    let policy = parse_policy(omni);

    let mut rt = Runtime::new();
    rt.update_data("items", serde_json::json!([1, 3]));

    let decision = rt.execute_policy(&policy);

    assert_eq!(decision.actions, vec!["ActOne".to_string()]);
    assert!(!decision.guard_triggered);
}
