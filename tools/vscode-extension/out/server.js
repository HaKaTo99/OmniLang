"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const node_1 = require("vscode-languageserver/node");
const vscode_languageserver_textdocument_1 = require("vscode-languageserver-textdocument");
// Simple keywords for completion. Extend as the language grows.
const KEYWORDS = [
    "POLICY",
    "RULE",
    "ACTION",
    "ALLOW",
    "DENY",
    "CHECK",
    "IF",
    "ELSE",
    "FOR",
    "WHILE",
    "IN",
    "MATCH",
    "WITH",
    "ACTOR",
    "TRIGGER",
];
const connection = (0, node_1.createConnection)(node_1.ProposedFeatures.all);
const documents = new node_1.TextDocuments(vscode_languageserver_textdocument_1.TextDocument);
connection.onInitialize((_params) => {
    return {
        capabilities: {
            textDocumentSync: node_1.TextDocumentSyncKind.Incremental,
            hoverProvider: true,
            completionProvider: {
                resolveProvider: false,
                triggerCharacters: [" ", "\n"],
            },
        },
    };
});
connection.onHover((_params) => {
    return {
        contents: {
            kind: "markdown",
            value: "**OmniLang** — validator intent + policy runner (IR/native/wasm).\n" +
                "Stdlib: math/string/time/collections/json/io/web/crypto/tensor.\n" +
                "Logging: timestamp/level/trace; metrics: Prometheus/OpenMetrics.",
        },
    };
});
connection.onCompletion((_params) => {
    return KEYWORDS.map((label) => ({ label, kind: node_1.CompletionItemKind.Keyword }));
});
async function validateTextDocument(textDocument) {
    const text = textDocument.getText();
    const diagnostics = [];
    if (text.trim().length === 0) {
        diagnostics.push({
            severity: node_1.DiagnosticSeverity.Warning,
            range: {
                start: { line: 0, character: 0 },
                end: { line: 0, character: 0 },
            },
            message: "Dokumen kosong",
            source: "omnilang-lsp",
        });
    }
    let balance = 0;
    for (const ch of text) {
        if (ch === "{")
            balance += 1;
        if (ch === "}")
            balance -= 1;
        if (balance < 0)
            break;
    }
    if (balance !== 0) {
        diagnostics.push({
            severity: node_1.DiagnosticSeverity.Warning,
            range: {
                start: { line: 0, character: 0 },
                end: { line: 0, character: 0 },
            },
            message: "Kurung kurawal tidak seimbang",
            source: "omnilang-lsp",
        });
    }
    connection.sendDiagnostics({ uri: textDocument.uri, diagnostics });
}
documents.onDidChangeContent((change) => {
    void validateTextDocument(change.document);
});
documents.onDidOpen((change) => {
    void validateTextDocument(change.document);
});
connection.onInitialized(() => {
    connection.console.info("OmniLang LSP initialized");
});
documents.listen(connection);
connection.listen();
