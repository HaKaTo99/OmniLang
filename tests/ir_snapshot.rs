use std::fs;

use omnilang_core::ir::build_policy_ir;
use omnilang_core::lexer::Lexer;
use omnilang_core::parser::Parser;
use serde_json::Value;

#[test]
fn demo_ir_matches_snapshot() {
    let source = fs::read_to_string("examples/demo.omni").expect("read demo.omni");

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("tokenize demo");
    let mut parser = Parser::new(tokens);
    let policy = parser.parse_policy().expect("parse demo");

    let ir = build_policy_ir(&policy);
    let produced = serde_json::to_value(&ir).expect("serialize ir");

    let snapshot_text = fs::read_to_string("tests/fixtures/demo_ir_snapshot.json")
        .expect("read snapshot");
    let snapshot: Value = serde_json::from_str(&snapshot_text).expect("parse snapshot");

    assert_eq!(produced, snapshot, "IR output should remain stable for demo.omni");
}
