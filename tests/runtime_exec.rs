use omnilang_core::{lexer::Lexer, parser::Parser, runtime::Runtime};

fn parse_policy(source: &str) -> omnilang_core::ast::Policy {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("lexing failed");
    let mut parser = Parser::new(tokens);
    parser.parse_policy().expect("parse failed")
}

#[test]
fn exec_demo_with_context_triggers_actions() {
    let omni_src = std::fs::read_to_string("examples/demo.omni").expect("demo.omni missing");
    let policy = parse_policy(&omni_src);

    let mut rt = Runtime::new();
    // context optional; ensures load works and guards remain off
    rt.load_context_from_file("examples/context.json").expect("context load failed");

    let decision = rt.execute_policy(&policy);

    assert!(!decision.actions.is_empty(), "expected at least one action to trigger");
    assert!(!decision.guard_triggered, "guards should not trigger for demo context");
    assert!(!decision.logs.is_empty(), "logs should be captured");
}
