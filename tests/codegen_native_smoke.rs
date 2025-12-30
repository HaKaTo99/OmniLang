use omnilang_core::{emitter::CompileTarget, emitter::emit, lexer::Lexer, parser::Parser};
use std::fs;

fn parse_policy(path: &str) -> omnilang_core::ast::Policy {
    let src = fs::read_to_string(path).expect("policy missing");
    let mut lexer = Lexer::new(&src);
    let tokens = lexer.tokenize().expect("lex failed");
    let mut parser = Parser::new(tokens);
    parser.parse_policy().expect("parse failed")
}

#[test]
fn native_codegen_produces_binary() {
    let policy = parse_policy("examples/demo.omni");
    let out_path = "target/test_native_runner.bin";
    emit(&policy, CompileTarget::Native, out_path).expect("emit native failed");
    let meta = fs::metadata(out_path).expect("output missing");
    assert!(meta.len() > 0, "native binary should be non-empty");
    fs::remove_file(out_path).ok();
}
