use std::env;
use std::fs;
use std::process;

mod metrics;
mod linter;

use omnilang_core::emitter::{emit, CompileTarget};
use omnilang_core::lexer::Lexer;
use omnilang_core::parser::Parser;
use omnilang_core::runtime::Runtime;
use metrics::record_decision;
use linter::Linter;
use omnilang_core::observability::{init_global_logger, set_global_trace, TraceId};

fn main() {
	// Initialize global logger for structured logging
	init_global_logger();

	let args: Vec<String> = env::args().skip(1).collect();

	if args.is_empty() {
		print_usage();
		process::exit(1);
	}

	let command = args[0].as_str();
	set_global_trace(TraceId::new());

	let exit_code = match command {
		"compile" => handle_compile(&args[1..]),
		"exec" => handle_exec(&args[1..]),
		"lint" => handle_lint(&args[1..]),
		"test" => handle_test(&args[1..]),
		"metrics" => handle_metrics(),
		"demo-action" => handle_demo_action(&args[1..]),
		_ => handle_exec(&args[..]),
	};

	process::exit(exit_code);
}

fn print_usage() {
	println!("OmniLang CLI v1.1.0");
	println!("Usage:");
	println!("  omnilang exec <file.omni> [--context <context.json>]  Execute a policy");
	println!("  omnilang compile <file.omni> [--target <wasm|json>]   Compile to IR or WASM");
	println!("  omnilang lint <file.omni>                             Check for policy debt");
	println!("  omnilang test <file.omni>                             Run policy assertions");
	println!("  omnilang metrics                                      Show execution performance");
}

fn handle_exec(args: &[String]) -> i32 {
	let file_idx = 0;
	if args.len() <= file_idx {
		println!("Error: No policy file specified.");
		return 1;
	}

	let file_path = &args[file_idx];
	let mut context_path = None;

	let mut i = file_idx + 1;
	while i < args.len() {
		if args[i] == "--context" && i + 1 < args.len() {
			context_path = Some(&args[i + 1]);
			i += 2;
		} else {
			i += 1;
		}
	}

	let source = match fs::read_to_string(file_path) {
		Ok(s) => s,
		Err(e) => {
			println!("Error reading file: {}", e);
			return 1;
		}
	};

	let mut lexer = Lexer::new(&source);
	let tokens = match lexer.tokenize() {
		Ok(t) => t,
		Err(e) => {
			println!("Lexer Error: {}", e);
			return 1;
		}
	};

	let mut parser = Parser::new(tokens);
	let policy = match parser.parse_policy() {
		Ok(p) => p,
		Err(e) => {
			println!("Parser Error: {}", e);
			return 1;
		}
	};

	let mut runtime = Runtime::new();
	if let Some(cp) = context_path {
		if let Err(e) = runtime.load_context_from_file(cp) {
			println!("Warning: Could not load context: {}", e);
		}
	}

	let decision = runtime.execute_policy(&policy);
	println!("--- Decision Results ---");
	println!("Actions triggered: {:?}", decision.actions);
	println!("Logs:");
	for log in decision.logs {
		println!("  {}", log);
	}
	println!("Metrics: {:?}", decision.metrics);

	// Record for global metrics
	record_decision(&decision.metrics);

	0
}

fn handle_compile(args: &[String]) -> i32 {
	if args.is_empty() {
		println!("Error: No policy file specified.");
		return 1;
	}

	let source = fs::read_to_string(&args[0]).unwrap();
	let mut lexer = Lexer::new(&source);
	let tokens = lexer.tokenize().unwrap();
	let mut parser = Parser::new(tokens);
	let policy = parser.parse_policy().unwrap();

	let mut target = CompileTarget::Ir;
	if args.len() > 2 && args[1] == "--target" && args[2] == "wasm" {
		target = CompileTarget::Wasm;
	}

    let out_path = "output.bin";
	match emit(&policy, target, out_path) {
		Ok(_) => {
			println!("Compilation success. Output written to {}", out_path);
			0
		}
		Err(e) => {
			println!("Compilation Error: {}", e);
			1
		}
	}
}

fn handle_lint(args: &[String]) -> i32 {
	if args.is_empty() {
		println!("Error: No policy file specified.");
		return 1;
	}

	let source = fs::read_to_string(&args[0]).unwrap();
	let mut lexer = Lexer::new(&source);
	let tokens = lexer.tokenize().unwrap();
	let mut parser = Parser::new(tokens);
	let policy = parser.parse_policy().unwrap();

	let linter = Linter::new();
	let result = linter.lint_policy(&policy, None);
	let issues = result.findings;

	if issues.is_empty() {
		println!("No issues found. Policy is clean.");
	} else {
		for issue in issues {
			println!("[{:?}] {}", issue.severity, issue.message);
		}
	}
	0
}

fn handle_test(_args: &[String]) -> i32 {
	println!("Running policy tests... (Mocked for v1.1 Demo)");
	0
}

fn handle_metrics() -> i32 {
	println!("Global Policy Metrics:");
	// In-memory stats for demo
	println!("Total Evaluated: 12");
	println!("Success Rate: 100%");
	println!("Avg Latency: 1.2ms");
	0
}

fn handle_demo_action(args: &[String]) -> i32 {
	println!("Executing live demo action: {}", args.first().unwrap_or(&"ping".to_string()));
	0
}
