use wasm_bindgen::prelude::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::program_evaluator::ProgramEvaluator;

#[wasm_bindgen]
pub fn run_omnilang(source: &str) -> String {
    let mut lexer = Lexer::new(source);
    match lexer.tokenize() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse_program() {
                Ok(ast) => {
                    let mut evaluator = ProgramEvaluator::new();
                    match evaluator.evaluate_program(&ast) {
                        Ok(_) => {
                            if evaluator.globals.contains_key("main") {
                                match evaluator.call_function_by_name("main", vec![]) {
                                    Ok(res) => format!("Eksekusi Berhasil! Output Evaluasi: {:?}", res),
                                    Err(e) => format!("Runtime Error: {}", e),
                                }
                            } else {
                                format!("Sintaks valid tersimpan di AST ({} Node ditemukan). Tidak ada fungsi main().", ast.modules.len())
                            }
                        }
                        Err(e) => format!("Runtime Error: {}", e),
                    }
                },
                Err(e) => format!("Syntax Error: {:?}", e),
            }
        },
        Err(e) => format!("Lexer Error: {:?}", e),
    }
}
