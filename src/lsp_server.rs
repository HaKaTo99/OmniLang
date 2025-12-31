// src/lsp_server.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write};

/// LSP Server implementation for OmniLang
/// Current status: Skeleton / MVP Handshake
pub struct LspServer {
    is_running: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LspMessage {
    pub jsonrpc: String,
    pub method: Option<String>,
    pub id: Option<Value>,
    pub params: Option<Value>,
}

impl LspServer {
    pub fn new() -> Self {
        LspServer { is_running: true }
    }

    pub fn start(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        while self.is_running {
            let mut line = String::new();
            // LSP uses Content-Length header, but we'll implement a basic line reader for skeleton tests
            if handle.read_line(&mut line)? == 0 {
                break;
            }

            if line.trim().is_empty() {
                continue;
            }

            self.handle_message(&line);
        }

        Ok(())
    }

    fn handle_message(&mut self, _raw: &str) {
        // Placeholder for JSON parsing and dispatch
        // In the future, this will link to parser::Parser and checker::Checker
        // to provide diagnostics and hover information.
    }

    pub fn respond(&self, id: Value, result: Value) {
        let response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        });
        let body = response.to_string();
        print!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        io::stdout().flush().unwrap();
    }
}

pub fn run_lsp_server() {
    let mut server = LspServer::new();
    if let Err(e) = server.start() {
        eprintln!("LSP Server error: {}", e);
    }
}