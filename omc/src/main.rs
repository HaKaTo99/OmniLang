mod lexer;
mod parser;
mod semantic;
mod ir;
mod backend;
mod ui;
mod compiler;

mod tui_app; // Phase 12

use ui::UI;
use compiler::Compiler;
use tui_app::App; // Phase 12
use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    // CLI Argument Parsing
    let args: Vec<String> = env::args().collect();
    let visual_mode = args.iter().any(|arg| arg == "--visual" || arg == "-v");

    UI::banner();
    
    // If visual mode, bypass CLI reporting and standard flow
    if visual_mode {
        // Find input file (excluding flags)
        let filename = args.iter().find(|arg| !arg.starts_with("-") && !arg.ends_with("omc.exe"));
        let input = if let Some(f) = filename {
            match fs::read_to_string(f) {
                Ok(c) => c,
                Err(_) => "fn main() { print(1); }".to_string(), // Fallback
            }
        } else {
             r#"fn factorial(n: int) {
    if (n < 2) { return 1; }
    return n * factorial(n - 1);
}
let res = factorial(5);
print(res);
"#.to_string()
        };

        let mut app = App::new(input);
        if let Err(e) = app.run() {
            println!("Error running TUI: {}", e);
        }
        return;
    }

    let (input, _filename) = if args.len() > 1 {
        let filename = &args[1];
        UI::info(&format!("Reading source file: {}", filename));
        match fs::read_to_string(filename) {
            Ok(content) => (content, filename.clone()),
            Err(e) => {
                UI::error(&format!("Failed to read file '{}': {}", filename, e));
                return;
            }
        }
    } else {
        UI::warning("No input file provided. Using built-in demo mode.");
        let demo = r#"fn factorial(n: int) {
    if (n < 2) { return 1; }
    return n * factorial(n - 1);
}
let res = factorial(5);
print(res);"#;
        (demo.to_string(), "demo.omni".to_string())
    };

    UI::section("SOURCE CODE");
    UI::code_block(&input);

    let start_time = Instant::now();

    // Run Compilation via Compiler Module
    UI::step(1, 5, "Compiling...");
    let result = Compiler::compile(&input);

    if !result.success {
        UI::error("Compilation Failed:");
        for err in result.errors {
            println!("  - {}", err);
        }
        return;
    }

    UI::step(2, 5, "AST & Semantic Analysis [OK]");
    UI::step(3, 5, "IR Generation [OK]");
    UI::step(4, 5, "Rust Backend [OK]");

    // Write Output
    let output_file = "output.rs";
    match fs::write(output_file, &result.rust_code) {
        Ok(_) => {
            UI::section("BUILD SUCCESSFUL");
            UI::success(&format!("Generated '{}'", output_file));
            
            let duration = start_time.elapsed();
            println!("\n  Time: {:.2?}", duration);
            println!("  Target: Rust (v1.0)");
            println!("  Status: Ready to compile with `rustc output.rs`");
        },
        Err(e) => UI::error(&format!("Failed to write output file: {}", e)),
    }
}

