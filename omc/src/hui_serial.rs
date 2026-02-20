// ─── HUI (Hardware User Interface) Module ────────────────────────────────
// Provides minimal serial/UART protocol for interacting with OmniLang
// on embedded devices, IoT gateways, and hardware panels.
//
// Protocol: Line-based text commands over stdin/stdout (or serial port)
// Commands:
//   EXEC <file.omni>     — Execute a policy file
//   EVAL <inline-code>   — Evaluate inline OmniLang code
//   LINT <file.omni>     — Lint a policy file
//   STATUS               — Get system status
//   PING                 — Health check
//   HELP                 — Show available commands
//   QUIT                 — Exit HUI mode
//
// Response format:
//   OK <result>          — Success
//   ERR <message>        — Error
//   DATA <json>          — Structured data response

use std::io::{self, BufRead, Write};
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::time::Instant;

const HUI_VERSION: &str = "1.0.0";
const HUI_PROMPT: &str = "HUI> ";

/// Find the Core Engine binary
fn find_engine() -> Option<PathBuf> {
    let candidates = vec![
        PathBuf::from("..").join("target").join("debug").join("omnilang.exe"),
        PathBuf::from("..").join("target").join("release").join("omnilang.exe"),
        PathBuf::from("target").join("debug").join("omnilang.exe"),
        PathBuf::from("target").join("release").join("omnilang.exe"),
    ];
    candidates.into_iter().find(|c| c.exists())
}

/// Run a Core Engine command and return the output
fn run_engine(command: &str, file_path: &str) -> Result<String, String> {
    let engine = find_engine().ok_or("Core Engine binary not found")?;
    let output = Command::new(&engine)
        .arg(command)
        .arg(file_path)
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout.trim().to_string())
    } else {
        Err(format!("{}\n{}", stdout.trim(), stderr.trim()))
    }
}

/// Process a single HUI command
fn process_command(line: &str) -> (bool, String) {
    let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
    let cmd = parts.get(0).map(|s| s.to_uppercase()).unwrap_or_default();
    let arg = parts.get(1).map(|s| s.trim()).unwrap_or("");

    match cmd.as_str() {
        "PING" => (false, "OK PONG".to_string()),

        "STATUS" => {
            let engine_status = if find_engine().is_some() { "available" } else { "not_found" };
            (false, format!(
                "DATA {{\"hui_version\":\"{}\",\"engine\":\"{}\",\"platform\":\"{}\"}}",
                HUI_VERSION, engine_status, std::env::consts::OS
            ))
        }

        "HELP" => {
            (false, "OK Commands: EXEC <file> | EVAL <code> | LINT <file> | STATUS | PING | HELP | QUIT".to_string())
        }

        "EXEC" => {
            if arg.is_empty() {
                return (false, "ERR Missing file path".to_string());
            }
            let start = Instant::now();
            match run_engine("exec", arg) {
                Ok(result) => {
                    let ms = start.elapsed().as_millis();
                    (false, format!("OK [{}ms] {}", ms, result.replace('\n', " | ")))
                }
                Err(e) => (false, format!("ERR {}", e.replace('\n', " | ")))
            }
        }

        "LINT" => {
            if arg.is_empty() {
                return (false, "ERR Missing file path".to_string());
            }
            match run_engine("lint", arg) {
                Ok(result) => (false, format!("OK {}", result.replace('\n', " | "))),
                Err(e) => (false, format!("ERR {}", e.replace('\n', " | ")))
            }
        }

        "EVAL" => {
            if arg.is_empty() {
                return (false, "ERR Missing inline code".to_string());
            }
            // Write to temp file and execute
            let tmp = std::env::temp_dir().join("hui_eval.omni");
            match fs::write(&tmp, arg) {
                Ok(_) => {
                    let start = Instant::now();
                    let result = run_engine("exec", &tmp.to_string_lossy());
                    let _ = fs::remove_file(&tmp);
                    let ms = start.elapsed().as_millis();
                    match result {
                        Ok(r) => (false, format!("OK [{}ms] {}", ms, r.replace('\n', " | "))),
                        Err(e) => (false, format!("ERR {}", e.replace('\n', " | ")))
                    }
                }
                Err(e) => (false, format!("ERR Failed to write temp file: {}", e))
            }
        }

        "QUIT" | "EXIT" => (true, "OK Goodbye".to_string()),

        "" => (false, String::new()),

        _ => (false, format!("ERR Unknown command: {}. Type HELP for available commands.", cmd)),
    }
}

/// Run the HUI interactive loop (reads from stdin, writes to stdout)
pub fn run_hui_mode() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    // Banner
    writeln!(out, "╔═══════════════════════════════════════╗").ok();
    writeln!(out, "║  OmniLang HUI v{} — Hardware Mode  ║", HUI_VERSION).ok();
    writeln!(out, "║  Serial/UART Protocol Interface      ║").ok();
    writeln!(out, "╚═══════════════════════════════════════╝").ok();
    writeln!(out, "Engine: {}", if find_engine().is_some() { "Ready" } else { "Not found" }).ok();
    writeln!(out, "Type HELP for commands, QUIT to exit.").ok();
    writeln!(out).ok();

    out.flush().ok();

    // Main loop — read lines from stdin
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                let (should_quit, response) = process_command(&input);
                if !response.is_empty() {
                    writeln!(out, "{}", response).ok();
                }
                if should_quit {
                    break;
                }
                write!(out, "{}", HUI_PROMPT).ok();
                out.flush().ok();
            }
            Err(_) => break,
        }
    }
}

/// Run in headless mode — execute a single file and return result code
/// This is designed for IoT/embedded deployment where no interactive terminal exists
pub fn run_headless(file_path: &str) -> i32 {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERR Gagal membaca file {}: {}", file_path, e);
            return 1;
        }
    };

    let command = if content.contains("module ") || content.contains("fn ") {
        "test"
    } else {
        "exec"
    };

    match run_engine(command, file_path) {
        Ok(result) => {
            println!("{}", result);
            0
        }
        Err(e) => {
            eprintln!("ERR {}", e);
            1
        }
    }
}
