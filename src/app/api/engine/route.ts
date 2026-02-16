import { NextResponse } from 'next/server';
import { exec } from 'child_process';
import { promisify } from 'util';
import path from 'path';
import fs from 'fs';
import os from 'os';

const execAsync = promisify(exec);

// Find the omnilang binary
function findBinary(): string {
    const root = path.resolve(process.cwd());
    const candidates = [
        path.join(root, 'target', 'debug', 'omnilang.exe'),
        path.join(root, 'target', 'release', 'omnilang.exe'),
        path.join(root, 'target', 'debug', 'omnilang'),
        path.join(root, 'target', 'release', 'omnilang'),
    ];
    for (const c of candidates) {
        if (fs.existsSync(c)) return c;
    }
    throw new Error('OmniLang Core Engine binary not found. Run `cargo build` in the root directory first.');
}

export async function POST(request: Request) {
    try {
        const body = await request.json();
        const { code, command = 'exec', context } = body;

        if (!code || typeof code !== 'string') {
            return NextResponse.json({ error: 'Missing or invalid "code" field' }, { status: 400 });
        }

        // Validate command
        const allowedCommands = ['exec', 'lint', 'compile'];
        if (!allowedCommands.includes(command)) {
            return NextResponse.json({ error: `Invalid command: ${command}. Allowed: ${allowedCommands.join(', ')}` }, { status: 400 });
        }

        // Write code to a temp file
        const tmpDir = os.tmpdir();
        const tmpFile = path.join(tmpDir, `omnilang_${Date.now()}.omni`);
        fs.writeFileSync(tmpFile, code, 'utf-8');

        // Optionally write context file
        let contextFile: string | null = null;
        if (context && typeof context === 'string') {
            contextFile = path.join(tmpDir, `omnilang_ctx_${Date.now()}.json`);
            fs.writeFileSync(contextFile, context, 'utf-8');
        }

        try {
            const binary = findBinary();
            let cmd = `"${binary}" ${command} "${tmpFile}"`;

            if (command === 'exec' && contextFile) {
                cmd += ` --context "${contextFile}"`;
            }

            if (command === 'compile') {
                cmd += ' --target wasm';
            }

            const { stdout, stderr } = await execAsync(cmd, { timeout: 10000 });

            return NextResponse.json({
                success: true,
                command,
                output: stdout.trim(),
                warnings: stderr.trim() || undefined,
                timestamp: new Date().toISOString(),
            });
        } finally {
            // Cleanup temp files
            try { fs.unlinkSync(tmpFile); } catch { }
            if (contextFile) {
                try { fs.unlinkSync(contextFile); } catch { }
            }
        }
    } catch (error: any) {
        // Distinguish between execution errors and system errors
        if (error.stdout || error.stderr) {
            return NextResponse.json({
                success: false,
                output: error.stdout?.trim() || '',
                error: error.stderr?.trim() || error.message,
                exitCode: error.code,
            }, { status: 200 }); // 200 because the API worked, the policy had issues
        }

        return NextResponse.json({
            success: false,
            error: error.message,
        }, { status: 500 });
    }
}

export async function GET() {
    return NextResponse.json({
        endpoints: {
            'POST /api/engine': {
                description: 'Execute OmniLang Core Engine commands',
                body: {
                    code: 'string (required) — OmniLang source code',
                    command: 'string (optional) — exec | lint | compile (default: exec)',
                    context: 'string (optional) — JSON context for policy evaluation',
                },
                examples: [
                    { command: 'exec', description: 'Execute policy and get decision results' },
                    { command: 'lint', description: 'Check policy for quality issues' },
                    { command: 'compile', description: 'Compile policy to IR/WASM' },
                ],
            },
        },
    });
}
