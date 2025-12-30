import {
  createConnection,
  Diagnostic,
  DiagnosticSeverity,
  InitializeParams,
  ProposedFeatures,
  TextDocuments,
  TextDocumentSyncKind,
  Hover,
  CompletionItem,
  CompletionItemKind,
} from "vscode-languageserver/node";
import { TextDocument } from "vscode-languageserver-textdocument";

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

const connection = createConnection(ProposedFeatures.all);
const documents: TextDocuments<TextDocument> = new TextDocuments(TextDocument);

connection.onInitialize((_params: InitializeParams) => {
  return {
    capabilities: {
      textDocumentSync: TextDocumentSyncKind.Incremental,
      hoverProvider: true,
      completionProvider: {
        resolveProvider: false,
        triggerCharacters: [" ", "\n"],
      },
    },
  };
});

// Basic hover: describe OmniLang intent validator focus.
connection.onHover((_params): Hover => {
  return {
    contents: {
      kind: "markdown",
      value:
        "**OmniLang** — validator intent + policy runner (IR/native/wasm).\n" +
        "Stdlib: math/string/time/collections/json/io/web/crypto/tensor.\n" +
        "Logging: timestamp/level/trace; metrics: Prometheus/OpenMetrics.",
    },
  };
});

// Keyword-based completion for now.
connection.onCompletion((_params): CompletionItem[] => {
  return KEYWORDS.map((label) => ({
    label,
    kind: CompletionItemKind.Keyword,
  }));
});

// Very lightweight diagnostics: empty file and unbalanced braces.
async function validateTextDocument(textDocument: TextDocument): Promise<void> {
  const text = textDocument.getText();
  const diagnostics: Diagnostic[] = [];

  if (text.trim().length === 0) {
    diagnostics.push({
      severity: DiagnosticSeverity.Warning,
      range: {
        start: { line: 0, character: 0 },
        end: { line: 0, character: 0 },
      },
      message: "Dokumen kosong", // notify empty document
      source: "omnilang-lsp",
    });
  }

  // Simple brace balance check to catch obvious structural issues.
  let balance = 0;
  for (const ch of text) {
    if (ch === "{") balance += 1;
    if (ch === "}") balance -= 1;
    if (balance < 0) break;
  }
  if (balance !== 0) {
    diagnostics.push({
      severity: DiagnosticSeverity.Warning,
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
