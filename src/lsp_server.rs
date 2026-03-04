// src/lsp_server.rs
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::collections::HashMap;
use tokio::sync::RwLock;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::checker::Checker;

fn extract_line_col_message(err: &str) -> (u32, u32, String) {
    if err.starts_with("[Line ") {
        if let Some(end_bracket) = err.find("] ") {
            let inner = &err[6..end_bracket];
            let parts: Vec<&str> = inner.split(", Col ").collect();
            if parts.len() == 2 {
                if let (Ok(line), Ok(col)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    let msg = err[end_bracket + 2..].to_string();
                    let lsp_line = if line > 0 { line - 1 } else { 0 };
                    let lsp_col = if col > 0 { col - 1 } else { 0 };
                    return (lsp_line, lsp_col, msg);
                }
            }
        }
    }
    (0, 0, err.to_string())
}

#[derive(Debug)]
pub struct Backend {
    client: Client,
    document_map: RwLock<HashMap<String, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "OmniLang LSP server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;
        self.document_map.write().await.insert(uri.clone(), text.clone());
        self.validate_document(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        if let Some(change) = params.content_changes.into_iter().last() {
            let text = change.text;
            self.document_map.write().await.insert(uri.clone(), text.clone());
            self.validate_document(&uri, &text).await;
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let _uri = params.text_document_position_params.text_document.uri.to_string();
        let _pos = params.text_document_position_params.position;
        // MVP: Provide basic hover info. Future versions will query the Semantic Analyzer (Checker).
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("OmniLang Token".to_string())),
            range: None,
        }))
    }
}

impl Backend {
    async fn validate_document(&self, uri_str: &str, text: &str) {
        let mut diagnostics = Vec::new();

        let mut lexer = Lexer::new(text);
        match lexer.tokenize() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                match parser.parse_program() {
                    Ok(program) => {
                        let mut checker = Checker::new();
                        if let Err(errors) = checker.check_program(&program) {
                            for err in errors {
                                let (err_line, err_col, msg) = extract_line_col_message(&err);
                                diagnostics.push(Diagnostic {
                                    range: Range::new(
                                        Position::new(err_line, err_col),
                                        Position::new(err_line, err_col + 1), // Optional spanning
                                    ),
                                    severity: Some(DiagnosticSeverity::ERROR),
                                    message: msg,
                                    ..Default::default()
                                });
                            }
                        }
                    }
                    Err(e) => {
                        let (err_line, err_col, msg) = extract_line_col_message(&e);
                        diagnostics.push(Diagnostic {
                            range: Range::new(
                                Position::new(err_line, err_col),
                                Position::new(err_line, err_col + 1),
                            ),
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: format!("Parser Error: {}", msg),
                            ..Default::default()
                        });
                    }
                }
            }
            Err(e) => {
                let (err_line, err_col, msg) = extract_line_col_message(&e);
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new(err_line, err_col),
                        Position::new(err_line, err_col + 1),
                    ),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("Lexer Error: {}", msg),
                    ..Default::default()
                });
            }
        }

        if let Ok(uri) = Url::parse(uri_str) {
            self.client.publish_diagnostics(uri, diagnostics, None).await;
        }
    }
}

pub async fn run_lsp_server_async() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: RwLock::new(HashMap::new()),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}