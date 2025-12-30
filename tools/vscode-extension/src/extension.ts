import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
  const serverModule = context.asAbsolutePath(path.join("out", "server.js"));

  const serverOptions: ServerOptions = {
    run: {
      command: process.execPath,
      args: [serverModule],
      transport: TransportKind.stdio,
    },
    debug: {
      command: process.execPath,
      args: ["--inspect=6009", serverModule],
      transport: TransportKind.stdio,
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "omni" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.omni"),
    },
  };

  client = new LanguageClient(
    "omnilangLsp",
    "OmniLang LSP",
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(client);
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
