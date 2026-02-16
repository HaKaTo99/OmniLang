use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::analyzer::SemanticAnalyzer;
use crate::ir::IRGenerator;
use crate::backend::RustGenerator;

pub struct Compiler;

pub struct CompilationResult {
    pub ast_debug: String,
    pub ir_output: Vec<String>,
    pub rust_code: String,
    pub errors: Vec<String>,
    pub success: bool,
}

impl Compiler {
    pub fn compile(input: &str) -> CompilationResult {
        let mut result = CompilationResult {
            ast_debug: String::new(),
            ir_output: Vec::new(),
            rust_code: String::new(),
            errors: Vec::new(),
            success: false,
        };

        // 0. Detect Declarative Policy (Intent)
        let trimmed = input.trim_start();
        if trimmed.starts_with("INTENT:") || trimmed.starts_with("RULE:") || trimmed.starts_with("POLICY:") {
            result.errors.push("ERR: Detected declarative policy file.".to_string());
            result.errors.push("Help: Use 'cargo run -- exec <file>' to evaluate this file with the Core Engine.".to_string());
            return result;
        }

        // 1. Lexing & Parsing
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        if !p.errors.is_empty() {
            result.errors = p.errors;
            return result;
        }
        result.ast_debug = "AST Generated Successfully".to_string(); // Placeholder for actual AST dump if needed

        // 2. Semantic Analysis
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze(&program);
        if !analyzer.errors.is_empty() {
            result.errors = analyzer.errors;
            return result;
        }

        // 3. IR Generation
        let mut ir_gen = IRGenerator::new();
        let instructions = ir_gen.generate(&program);
        for instr in &instructions {
            result.ir_output.push(format!("{}", instr));
        }

        // 4. Rust Generation
        let mut rust_gen = RustGenerator::new();
        result.rust_code = rust_gen.generate(&instructions);
        result.success = true;

        result
    }
}
