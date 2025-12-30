use std::env;
use std::fs;
use std::process;

mod ast;
// mod checker;
mod lexer;
mod parser;
// mod types;
mod evaluator;
mod program_evaluator;
mod error;
mod omniroutine;
mod runtime;
mod ir;
mod emitter;
mod codegen;
mod action_abi;
mod metrics;
mod observability;
mod linter;
mod ir_interpreter;
mod cache;

use action_abi::ActionResult;
use emitter::{emit, CompileTarget};
use error::OmniError;
use lexer::Lexer;
use parser::Parser;
use runtime::Runtime;
use metrics::record_decision;
use linter::{Linter, Severity};
use observability::{format_log, init_global_logger, set_global_trace, TraceId};
use serde_json::json;
use ir_interpreter::execute_ir;
use std::path::PathBuf;
use std::time::Duration;
use cache::EvalCache;

fn main() {
	// Initialize global logger for structured logging
	init_global_logger();

	let args: Vec<String> = env::args().skip(1).collect();

	if args.is_empty() {
		print_usage();
		process::exit(1);
	}

	let command = args[0].as_str();

	// Set global trace for this execution
	set_global_trace(TraceId::new());

	let exit_code = match command {
		"compile" => handle_compile(&args[1..]),
		"exec" => handle_exec(&args[1..], true),
		"lint" => handle_lint(&args[1..]),
		"test" => handle_test(&args[1..]),
		"metrics" => handle_metrics(),
		"demo-action" => handle_demo_action(&args[1..]),
		_ => handle_exec(&args[..], false),
	};

	process::exit(exit_code);
}

fn handle_compile(args: &[String]) -> i32 {
	if args.is_empty() {
		eprintln!("Usage: omnilang compile <file.omni> [--out path] [--target ir|native|wasm]");
		return 1;
	}

	let filename = args[0].clone();
	let mut out_path = "target/policy_ir.json".to_string();
	let mut target = CompileTarget::Ir;

	let mut i = 1;
	while i < args.len() {
		if args[i] == "--out" && i + 1 < args.len() {
			out_path = args[i + 1].clone();
			i += 2;
		} else if args[i] == "--target" && i + 1 < args.len() {
			let t = args[i + 1].as_str();
			target = match t {
				"ir" => CompileTarget::Ir,
				"native" => CompileTarget::Native,
				"wasm" => CompileTarget::Wasm,
				_ => {
					eprintln!("[ERROR] target tidak dikenal: {} (gunakan ir|native|wasm)", t);
					return 1;
				}
			};
			i += 2;
		} else {
			i += 1;
		}
	}

	println!("Compiling {} to target {:?}...", filename, target);
	let policy = match parse_policy_from_file(&filename) {
		Ok(p) => p,
		Err(e) => {
			eprintln!("[ERROR] {}", e);
			return 1;
		}
	};

	if let Err(e) = emit(&policy, target, &out_path) {
		eprintln!("[ERROR] {}", e);
		return 1;
	}

	println!("[OK] output ditulis ke {}", out_path);
	0
}

fn handle_exec(args: &[String], explicit_exec: bool) -> i32 {
	let opts = match parse_exec_options(args) {
		Ok(v) => v,
		Err(usage) => {
			if explicit_exec {
				eprintln!("{}", usage);
			} else {
				print_usage();
			}
			return 1;
		}
	};

	let policy: Option<ast::Policy> = if opts.ir_path.is_none() {
		match parse_policy_from_file(&opts.filename) {
			Ok(p) => Some(p),
			Err(e) => {
				eprintln!("[ERROR] {}", e);
				return 1;
			}
		}
	} else {
		None
	};

	let ir_policy: Option<ir::PolicyIR> = if let Some(ref ir_path) = opts.ir_path {
		match load_ir_from_file(ir_path) {
			Ok(ir) => Some(ir),
			Err(e) => {
				eprintln!("[ERROR] {}", e);
				return 1;
			}
		}
	} else {
		None
	};

	let mut runtime = Runtime::new();

	if let Some(ctx_path) = opts.context_path.clone() {
		if let Err(e) = runtime.load_context_from_file(&ctx_path) {
			eprintln!("Warning: {}", e);
		}
	}

	let cache = EvalCache::new(
		PathBuf::from(opts.cache_file.clone()),
		Duration::from_millis(opts.cache_ttl_ms),
		opts.cache_enabled,
	);

	let cache_key = if opts.cache_enabled {
		some_key_for_cache(&opts, &runtime)
	} else {
		None
	};

	if let (true, Some(ref key)) = (opts.cache_enabled, &cache_key) {
		if let Some(dec) = cache.get(key) {
			print_decision(&dec, opts.trace, opts.trace_json.as_deref());
			return 0;
		}
	}

	let decision = if let Some(ref irp) = ir_policy {
		execute_ir(irp, runtime.context_snapshot())
	} else if let Some(ref p) = policy {
		runtime.execute_policy(p)
	} else {
		eprintln!("[ERROR] No policy or IR provided");
		return 1;
	};
	print_decision(&decision, opts.trace, opts.trace_json.as_deref());
	if let (true, Some(ref key)) = (opts.cache_enabled, &cache_key) {
		cache.put(key, &decision);
	}
	record_decision(&decision.metrics);
	0
}

fn handle_lint(args: &[String]) -> i32 {
	if args.is_empty() {
		eprintln!("Usage: omnilang lint <file.omni>");
		return 1;
	}
	let filename = args[0].clone();
	let policy = match parse_policy_from_file(&filename) {
		Ok(p) => p,
		Err(e) => {
			eprintln!("Lint failed: {}", e);
			return 1;
		}
	};

	let linter = Linter::new();
	let result = linter.lint_policy(&policy, None);

	for f in &result.findings {
		match f.severity {
			Severity::Error => eprintln!("error: {}", f.message),
			Severity::Warning => println!("warning: {}", f.message),
			Severity::Info => println!("info: {}", f.message),
		}
	}

	if result.has_errors { 1 } else { 0 }
}

fn handle_test(args: &[String]) -> i32 {
	let opts = match parse_exec_options(args) {
		Ok(v) => v,
		Err(usage) => {
			eprintln!("{}", usage);
			return 1;
		}
	};

	let policy: Option<ast::Policy> = if opts.ir_path.is_none() {
		match parse_policy_from_file(&opts.filename) {
			Ok(p) => Some(p),
			Err(e) => {
				eprintln!("[ERROR] {}", e);
				return 1;
			}
		}
	} else {
		None
	};

	let ir_policy: Option<ir::PolicyIR> = if let Some(ref ir_path) = opts.ir_path {
		match load_ir_from_file(ir_path) {
			Ok(ir) => Some(ir),
			Err(e) => {
				eprintln!("[ERROR] {}", e);
				return 1;
			}
		}
	} else {
		None
	};

	let mut runtime = Runtime::new();
	if let Some(ctx_path) = opts.context_path.clone() {
		if let Err(e) = runtime.load_context_from_file(&ctx_path) {
			eprintln!("Warning: {}", e);
		}
	}

	let cache = EvalCache::new(
		PathBuf::from(opts.cache_file.clone()),
		Duration::from_millis(opts.cache_ttl_ms),
		opts.cache_enabled,
	);
	let cache_key = if opts.cache_enabled {
		some_key_for_cache(&opts, &runtime)
	} else {
		None
	};
	if let (true, Some(ref key)) = (opts.cache_enabled, &cache_key) {
		if let Some(dec) = cache.get(key) {
			print_decision(&dec, opts.trace, opts.trace_json.as_deref());
			println!("Metrics: rules={}, actions={}, guards={}, duration_ms={}",
				dec.metrics.rules_evaluated,
				dec.metrics.actions_triggered,
				dec.metrics.guard_hits,
				dec.metrics.duration_ms);
			record_decision(&dec.metrics);
			return 0;
		}
	}

	let decision = if let Some(ref irp) = ir_policy {
		execute_ir(irp, runtime.context_snapshot())
	} else if let Some(ref p) = policy {
		runtime.execute_policy(p)
	} else {
		eprintln!("[ERROR] No policy or IR provided");
		return 1;
	};
	print_decision(&decision, opts.trace, opts.trace_json.as_deref());
	println!("Metrics: rules={}, actions={}, guards={}, duration_ms={}",
		decision.metrics.rules_evaluated,
		decision.metrics.actions_triggered,
		decision.metrics.guard_hits,
		decision.metrics.duration_ms);
	if let (true, Some(ref key)) = (opts.cache_enabled, &cache_key) {
		cache.put(key, &decision);
	}
	record_decision(&decision.metrics);
	0
}

fn handle_metrics() -> i32 {
	let snapshot = metrics::export_openmetrics();
	println!("{}", snapshot);
	0
}

// Placeholder for demo action handler; implement real adapter as needed.
fn handle_demo_action(args: &[String]) -> i32 {
	if args.is_empty() {
		eprintln!("Usage: omnilang demo-action <action_name> [--fail]");
		return 1;
	}

	let action_name = args[0].clone();
	let simulate_fail = args.iter().any(|a| a == "--fail");

	let runtime = Runtime::new();
	let results = runtime.execute_actions_with_routine(&[action_name.clone()], 1, |action, _ctx| {
		if simulate_fail {
			Err(OmniError::InvalidInput("simulated failure".to_string()))
		} else {
			Ok(json!({"executed": action}))
		}
	});

	for res in results {
		match res {
			ActionResult::Success { output, elapsed_ms } => {
				println!("{}", format_log(&format!(
					"action={} status=success elapsed_ms={} output={:?}",
					action_name,
					elapsed_ms.unwrap_or(0),
					output
				)));
			}
			ActionResult::Failed { error, elapsed_ms } => {
				println!("{}", format_log(&format!(
					"action={} status=failed elapsed_ms={} error={}",
					action_name,
					elapsed_ms.unwrap_or(0),
					error
				)));
			}
		}
	}

	0
}

fn handle_run(args: &[String]) -> i32 {
	if args.is_empty() {
		eprintln!("Usage: omnilang run <file.omni>");
		return 1;
	}

	let filename = args[0].clone();

	let program = match parse_program_from_file(&filename) {
		Ok(p) => p,
		Err(e) => {
			eprintln!("[ERROR] {}", e);
			return 1;
		}
	};

	let mut evaluator = program_evaluator::ProgramEvaluator::new();
	match evaluator.evaluate_program(&program) {
		Ok(result) => {
			println!("Result: {:?}", result);
			0
		}
		Err(e) => {
			eprintln!("[ERROR] {}", e);
			1
		}
	}
}

fn parse_program_from_file(filename: &str) -> Result<ast::Program, String> {
	let source_code = fs::read_to_string(filename)
		.map_err(|err| format!("Error reading file {}: {}", filename, err))?;

	let mut lexer = Lexer::new(&source_code);
	let tokens = lexer
		.tokenize()
		.map_err(|msg| format!("Lexer Error: {}", msg))?;

	let mut parser_inst = Parser::new(tokens);
	let program = parser_inst
		.parse_program()
		.map_err(|msg| format!("Parsing Error: {}", msg))?;

	Ok(program)
}

fn parse_policy_from_file(filename: &str) -> Result<ast::Policy, String> {
	let source_code = fs::read_to_string(filename)
		.map_err(|err| format!("Error reading file {}: {}", filename, err))?;

	let mut lexer = Lexer::new(&source_code);
	let tokens = lexer
		.tokenize()
		.map_err(|msg| format!("Lexer Error: {}", msg))?;

	let mut parser_inst = Parser::new(tokens);
	let policy = parser_inst
		.parse_policy()
		.map_err(|msg| format!("Parsing Error: {}", msg))?;

	Ok(policy)
}

fn load_ir_from_file(path: &str) -> Result<ir::PolicyIR, String> {
	let raw = fs::read_to_string(path)
		.map_err(|err| format!("Error reading IR {}: {}", path, err))?;
	serde_json::from_str::<ir::PolicyIR>(&raw)
		.map_err(|err| format!("Invalid IR JSON {}: {}", path, err))
}

#[derive(Debug, Clone)]
struct ExecOptions {
	filename: String,
	context_path: Option<String>,
	trace: bool,
	trace_json: Option<String>,
	cache_enabled: bool,
	cache_ttl_ms: u64,
	cache_file: String,
	cache_key: Option<String>,
	ir_path: Option<String>,
}

fn parse_exec_options(args: &[String]) -> Result<ExecOptions, String> {
	if args.is_empty() {
		return Err("Usage: omnilang exec|test <file.omni> [--context ctx.json] [--trace] [--trace-json path] [--cache-ttl-ms N] [--cache-key str] [--cache-file path] [--use-ir path]".to_string());
	}
	let filename = args[0].clone();
	let mut context_path: Option<String> = None;
	let mut trace = false;
	let mut trace_json: Option<String> = None;
	let mut cache_enabled = false;
	let mut cache_ttl_ms: u64 = 0;
	let mut cache_file: String = "target/cache/eval_cache.json".to_string();
	let mut cache_key: Option<String> = None;
	let mut ir_path: Option<String> = None;
	let mut i = 1;
	while i < args.len() {
		match args[i].as_str() {
			"--context" => {
				if i + 1 < args.len() {
					context_path = Some(args[i + 1].clone());
					i += 2;
				} else {
					return Err("Missing value for --context".to_string());
				}
			}
			"--trace" => {
				trace = true;
				i += 1;
			}
			"--trace-json" => {
				if i + 1 < args.len() {
					trace_json = Some(args[i + 1].clone());
					trace = true;
					i += 2;
				} else {
					return Err("Missing value for --trace-json".to_string());
				}
			}
			"--cache-ttl-ms" => {
				if i + 1 < args.len() {
					cache_ttl_ms = args[i + 1].parse::<u64>().map_err(|_| "Invalid --cache-ttl-ms".to_string())?;
					cache_enabled = cache_ttl_ms > 0;
					i += 2;
				} else {
					return Err("Missing value for --cache-ttl-ms".to_string());
				}
			}
			"--cache-file" => {
				if i + 1 < args.len() {
					cache_file = args[i + 1].clone();
					i += 2;
				} else {
					return Err("Missing value for --cache-file".to_string());
				}
			}
			"--cache-key" => {
				if i + 1 < args.len() {
					cache_key = Some(args[i + 1].clone());
					cache_enabled = true;
					i += 2;
				} else {
					return Err("Missing value for --cache-key".to_string());
				}
			}
			"--use-ir" => {
				if i + 1 < args.len() {
					ir_path = Some(args[i + 1].clone());
					i += 2;
				} else {
					return Err("Missing value for --use-ir".to_string());
				}
			}
			_ => {
				i += 1;
			}
		}
	}
	Ok(ExecOptions {
		filename,
		context_path,
		trace,
		trace_json,
		cache_enabled,
		cache_ttl_ms,
		cache_file,
		cache_key,
		ir_path,
	})
}

fn print_decision(decision: &runtime::Decision, trace: bool, trace_json: Option<&str>) {
	for log in &decision.logs {
		println!("- {}", format_log(log));
	}
	if decision.guard_triggered {
		println!("{}", format_log("Guard triggered during execution"));
	}
	if decision.actions.is_empty() {
		println!("{}", format_log("No actions triggered. System is compliant/safe."));
	} else {
		println!("{}", format_log("ACTION REQUIRED:"));
		for action in &decision.actions {
			println!("  -> EXECUTE: {}", format_log(action));
		}
	}

	if trace {
		println!("{}", format_log("TRACE VIEW:"));
		for evt in &decision.traces {
			let ctx_snip = evt
				.context_snapshot
				.as_ref()
				.map(|v| serde_json::to_string(v).unwrap_or_else(|_| "<ctx>".to_string()))
				.unwrap_or_else(|| "<none>".to_string());
			println!(
				"  step={} phase={} t={}ms msg={} ctx={}",
				evt.step, evt.phase, evt.elapsed_ms, evt.message, ctx_snip
			);
		}
	}

	if let Some(path) = trace_json {
		if let Ok(blob) = serde_json::to_string_pretty(decision) {
			if let Err(err) = std::fs::write(path, blob) {
				eprintln!("failed to write trace json: {}", err);
			} else {
				println!("{}", format_log(&format!("Trace JSON written to {}", path)));
			}
		}
	}
}

fn some_key_for_cache(opts: &ExecOptions, runtime: &Runtime) -> Option<String> {
	if let Some(key) = &opts.cache_key {
		return Some(key.clone());
	}
	// default: hash filename/ir + context snapshot
	let mut hasher = std::collections::hash_map::DefaultHasher::new();
	use std::hash::Hash;
	use std::hash::Hasher;
	opts.filename.hash(&mut hasher);
	if let Some(ir) = &opts.ir_path {
		ir.hash(&mut hasher);
	}
	let ctx_str = format!("{}", runtime.context_data_as_string());
	ctx_str.hash(&mut hasher);
	Some(format!("cache-{:x}", hasher.finish()))
}

fn print_usage() {
	eprintln!("Usage:");
	eprintln!("  omnilang compile <file.omni> [--out path] [--target ir|native|wasm]");
	eprintln!("  omnilang exec <file.omni> [--context ctx.json] [--trace] [--trace-json path] [--cache-ttl-ms N] [--cache-key str] [--cache-file path] [--use-ir path]");
	eprintln!("  omnilang run <file.omni>");
	eprintln!("  omnilang test <file.omni> [--context ctx.json] [--trace] [--trace-json path] [--cache-ttl-ms N] [--cache-key str] [--cache-file path] [--use-ir path]");
	eprintln!("  omnilang lint <file.omni>");
	eprintln!("  omnilang metrics");
	eprintln!("  omnilang demo-action <action_name> [--param key=value ...]");
}
