import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
    Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    // Determine the OmniLang executable path (assuming it's in PATH or at a specific relative location)
    // For local development, point directly to the cargo build output
    let command = process.platform === 'win32' ? 'omnilang.exe' : 'omnilang';

    // In production, users should have "omnilang" in their PATH or configured via settings
    let defaultBinPath = path.join(context.extensionPath, '..', '..', 'target', 'debug', command);

    let serverExecutable: Executable = {
        command: workspace.getConfiguration('omnilang').get('executablePath', defaultBinPath),
        args: ['--lsp'],
        options: { env: process.env }
    };

    let serverOptions: ServerOptions = {
        run: serverExecutable,
        debug: serverExecutable
    };

    let clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'omnilang' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
        }
    };

    client = new LanguageClient(
        'omnilangLanguageServer',
        'OmniLang Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
