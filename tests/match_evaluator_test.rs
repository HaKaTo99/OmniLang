use omnilang_core::lexer::Lexer;
use omnilang_core::parser::Parser;
use omnilang_core::program_evaluator::{ProgramEvaluator, Value};
use omnilang_core::ast::Item;

#[test]
fn test_match_in_module_simple() {
    let source = "module T { const r: i32 = match 1 { 1 => 10, _ => 20 }; }";
    
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().expect("Lexing failed");
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => panic!("PARSER ERROR: {}", e),
    };

    let mut evaluator = ProgramEvaluator::new();
    if let Item::Const(c) = &program.modules[0].items[0] {
        let value = evaluator.evaluate_expression(&c.value).unwrap();
        match value {
            Value::Number(n) => assert_eq!(n, 10.0),
            _ => panic!("Expected 10.0, got {:?}", value),
        }
    }
}
