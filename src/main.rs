use std::env;
use std::fs;
use std::process;

mod ast;
// mod checker;
mod lexer;
mod parser;
// mod types;
mod evaluator;
mod runtime;

use lexer::Lexer;
// use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: omnilang <filename.omni>");
        process::exit(1);
    }

    let filename = &args[1];
    println!("Compiling {}...", filename);

    let source_code = match fs::read_to_string(filename) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error reading file {}: {}", filename, err);
            process::exit(1);
        }
    };

    // 1. Lexing
    println!("Step 1: Lexing...");
    let mut lexer = Lexer::new(&source_code);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(msg) => {
            eprintln!("Lexer Error: {}", msg);
            process::exit(1);
        }
    };
    // println!("Tokens: {:?}", tokens); // Debug

    // 2. Parsing
    println!("Step 2: Parsing...");
    let mut parser_inst = parser::Parser::new(tokens);
    let policy = match parser_inst.parse_policy() {
        Ok(pol) => pol,
        Err(msg) => {
            eprintln!("Parsing Error: {}", msg);
            process::exit(1);
        }
    };

    println!("Parsing Successful! Policy Document Structure:");
    // println!("{:#?}", policy); // Optional: print AST

    // 3. Execution (Runtime)
    println!("\nStep 3: Initializing OmniLang Runtime...");
    let mut runtime = runtime::Runtime::new();

    // Simulasi Data Sensor
    println!("> Simulating Sensor Input: Distance=0.5, Suhu=52.0, Status=1.0, Mode=1.0");
    runtime.update_data("Distance", 0.5);
    runtime.update_data("Suhu", 52.0);
    runtime.update_data("Status", 1.0);
    runtime.update_data("Mode", 1.0);

    // Jalankan Policy Engine
    let actions = runtime.execute_policy(&policy);

    println!("\nStep 4: Execution Result");
    if actions.is_empty() {
        println!("No actions triggered. System is compliant/safe.");
    } else {
        println!("⚠️  ACTION REQUIRED:");
        for action in actions {
            println!("  -> EXECUTE: {}", action);
        }
    }
}
