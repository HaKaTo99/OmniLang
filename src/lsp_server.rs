// src/lsp_server.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, Write, Read};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::checker::Checker;
use std::collections::HashMap;

/// LSP Server implementation for OmniLang
pub struct LspServer {
    is_running: bool,
    documents: HashMap<String, String>,
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
        LspServer { 
            is_running: true,
            documents: HashMap::new(),
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut reader = stdin.lock();

        loop {
            // 1. Read Content-Length header
            let mut size = 0;
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let parts: Vec<&str> = line.trim().split(": ").collect();
                    if parts.len() == 2 && parts[0] == "Content-Length" {
                         size = parts[1].parse().unwrap_or(0);
                    }
                }
                Err(_) => break,
            }
            
            // Read other headers (expect \r\n separator)
            while let Ok(n) = reader.read_line(&mut line) {
                if n == 0 || line.trim().is_empty() { break; }
                line.clear();
            }

            if size > 0 {
                let mut buf = vec![0u8; size];
                if reader.read_exact(&mut buf).is_ok() {
                    if let Ok(msg_str) = String::from_utf8(buf) {
                         self.handle_message(&msg_str);
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_message(&mut self, raw: &str) {
        let msg: Result<LspMessage, _> = serde_json::from_str(raw);
        if let Ok(msg) = msg {
            if let Some(method) = &msg.method {
                match method.as_str() {
                    "initialize" => {
                        let result = serde_json::json!({
                            "capabilities": {
                                "textDocumentSync": 1, // Full sync
                                "hoverProvider": true,
                                // "definitionProvider": true // Disabled until implemented
                            }
                        });
                        if let Some(id) = msg.id {
                            self.respond(id, result);
                        }
                    }
                    "textDocument/didOpen" => {
                        if let Some(params) = msg.params {
                            if let Some(text_document) = params.get("textDocument") {
                                if let (Some(uri), Some(text)) = (text_document.get("uri").and_then(|v| v.as_str()), text_document.get("text").and_then(|v| v.as_str())) {
                                    self.documents.insert(uri.to_string(), text.to_string());
                                    self.validate_document(uri);
                                }
                            }
                        }
                    }
                    "textDocument/didChange" => {
                        if let Some(params) = msg.params {
                             if let Some(text_document) = params.get("textDocument") {
                                if let Some(uri) = text_document.get("uri").and_then(|v| v.as_str()) {
                                    if let Some(changes) = params.get("contentChanges").and_then(|v| v.as_array()) {
                                        if let Some(last_change) = changes.last() {
                                            if let Some(text) = last_change.get("text").and_then(|v| v.as_str()) {
                                                self.documents.insert(uri.to_string(), text.to_string());
                                                self.validate_document(uri);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "shutdown" => {
                        if let Some(id) = msg.id {
                            self.respond(id, serde_json::Value::Null);
                        }
                    }
                    "exit" => {
                        self.is_running = false;
                        std::process::exit(0);
                    }
                    _ => {
                        // Ignore other methods for now
                    }
                }
            }
        }
    }

    fn validate_document(&self, uri: &str) {
        if let Some(text) = self.documents.get(uri) {
            let mut diagnostics = Vec::new();

            // Run Lexer
            let mut lexer = Lexer::new(text);
            match lexer.tokenize() {
                Ok(tokens) => {
                    // Run Parser
                    let mut parser = Parser::new(tokens);
                    match parser.parse_program() {
                        Ok(program) => {
                            // Run Checker
                            let mut checker = Checker::new();
                            if let Err(errors) = checker.check_program(&program) {
                                for err in errors {
                                    // Checker errors are just strings for now, no line info exposed easily
                                    // TODO: Improve checker error reporting to include line numbers
                                    diagnostics.push(serde_json::json!({
                                        "range": {
                                            "start": { "line": 0, "character": 0 },
                                            "end": { "line": 0, "character": 0 }
                                        },
                                        "severity": 1, // Error
                                        "message": err
                                    }));
                                }
                            }
                        }
                        Err(e) => {
                            // Parser error (string)
                            diagnostics.push(serde_json::json!({
                                "range": {
                                    "start": { "line": 0, "character": 0 },
                                    "end": { "line": 0, "character": 0 }
                                },
                                "severity": 1, 
                                "message": format!("Parser Error: {}", e)
                            }));
                        }
                    }
                }
                Err(e) => {
                     // Lexer error
                     diagnostics.push(serde_json::json!({
                        "range": {
                            "start": { "line": 0, "character": 0 },
                            "end": { "line": 0, "character": 0 }
                        },
                        "severity": 1, 
                        "message": format!("Lexer Error: {}", e)
                    }));
                }
            }

            self.publish_diagnostics(uri, diagnostics);
        }
    }

    fn publish_diagnostics(&self, uri: &str, diagnostics: Vec<Value>) {
        let params = serde_json::json!({
            "uri": uri,
            "diagnostics": diagnostics
        });
        
        // Notification message (no id)
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "textDocument/publishDiagnostics",
            "params": params
        });
        
        let body = notification.to_string();
        print!("Content-Length: {}\r\n\r\n{}", body.len(), body);
        io::stdout().flush().unwrap();
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