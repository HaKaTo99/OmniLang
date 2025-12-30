use omnilang_core::{emitter::{emit, CompileTarget}, lexer::Lexer, parser::Parser};
use std::fs;
use std::process::Command;

fn parse_policy(path: &str) -> omnilang_core::ast::Policy {
    let src = fs::read_to_string(path).expect("policy missing");
    let mut lexer = Lexer::new(&src);
    let tokens = lexer.tokenize().expect("lex failed");
    let mut parser = Parser::new(tokens);
    parser.parse_policy().expect("parse failed")
}

fn has_wasm_target() -> bool {
    if let Ok(output) = Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()
    {
        if let Ok(list) = String::from_utf8(output.stdout) {
            return list.lines().any(|l| l.trim() == "wasm32-wasi");
        }
    }
    false
}

#[test]
fn wasm_codegen_produces_binary() {
    if !has_wasm_target() {
        eprintln!("skip wasm smoke test: wasm32-wasi target not installed");
        return;
    }

    let policy = parse_policy("examples/demo.omni");
    let out_path = "target/test_wasm_runner.wasm";
    emit(&policy, CompileTarget::Wasm, out_path).expect("emit wasm failed");
    let meta = fs::metadata(out_path).expect("output missing");
    assert!(meta.len() > 0, "wasm binary should be non-empty");
    fs::remove_file(out_path).ok();
}
