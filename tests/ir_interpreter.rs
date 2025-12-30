use omnilang_core::{ir::build_policy_ir, ir_interpreter::execute_ir, lexer::Lexer, parser::Parser};
use serde_json::Value;

fn parse_policy(source: &str) -> omnilang_core::ast::Policy {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("lexing failed");
    let mut parser = Parser::new(tokens);
    parser.parse_policy().expect("parse failed")
}

fn load_context(path: &str) -> Value {
    let content = std::fs::read_to_string(path).expect("context missing");
    serde_json::from_str(&content).expect("invalid context json")
}

#[test]
fn execute_ir_runs_demo_policy() {
    let omni_src = std::fs::read_to_string("examples/demo.omni").expect("demo.omni missing");
    let policy = parse_policy(&omni_src);
    let ir = build_policy_ir(&policy);
    let ctx = load_context("examples/context.json");

    let decision = execute_ir(&ir, ctx);

    assert!(
        !decision.actions.is_empty(),
        "expected at least one action to trigger"
    );
    assert!(
        !decision.guard_triggered,
        "guards should not trigger for demo context"
    );
}
