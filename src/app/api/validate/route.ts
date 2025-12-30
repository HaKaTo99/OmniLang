import { NextResponse } from "next/server";
import { parseOmniLang } from "@/lib/omniling-parser";
import { recordError, recordParseError, recordRateLimited, recordRequest, resetTelemetry } from "@/lib/telemetry";
import { promises as fs } from "fs";
import os from "os";
import path from "path";
import { execFile } from "child_process";
import { promisify } from "util";
import { randomUUID } from "crypto";
import { rateBuckets, resetRateBuckets } from "./rate-limit-store";
import { logJSON } from "@/lib/logging";
import { beginRequest, endRequest, resetInflight } from "./inflight-store";
import {
  DISABLE_PY_FALLBACK,
  MAX_BODY_BYTES,
  MAX_CODE_BYTES,
  MAX_CONTEXT_BYTES,
  MAX_INFLIGHT,
  RATE_LIMIT_MAX_REQUESTS,
  RATE_LIMIT_WINDOW_MS,
} from "./config";

const execFileAsync = promisify(execFile);

function getClientIp(req: Request): string {
  return (
    req.headers.get("x-forwarded-for")?.split(",")[0].trim() ||
    req.headers.get("x-real-ip") ||
    "local"
  );
}

function isRateLimited(ip: string): boolean {
  const now = Date.now();
  const windowStart = now - RATE_LIMIT_WINDOW_MS;
  const bucket = rateBuckets.get(ip)?.filter((ts) => ts >= windowStart) || [];
  if (bucket.length >= RATE_LIMIT_MAX_REQUESTS) {
    rateBuckets.set(ip, bucket);
    return true;
  }
  bucket.push(now);
  rateBuckets.set(ip, bucket);
  return false;
}

let inflight = 0;

interface RuleSummary {
  condition: string;
  action: string;
}

function extractRules(code: string): RuleSummary[] {
  const rules: RuleSummary[] = [];
  const ruleRegex = /^-\s*IF\s+(.+?)\s+THEN\s+(.+)$/i;
  for (const line of code.split("\n")) {
    const match = line.trim().match(ruleRegex);
    if (match) {
      rules.push({ condition: match[1].trim(), action: match[2].trim() });
    }
  }
  return rules;
}

async function runRustEvaluator(code: string, context?: string) {
  const tmpDir = await fs.mkdtemp(path.join(os.tmpdir(), "omnilang-"));
  const filePath = path.join(tmpDir, "input.omni");
  await fs.writeFile(filePath, code, "utf8");

  const args = ["run", "--quiet", "--", "exec", filePath];
  let contextPath: string | undefined;
  if (context && context.trim().length > 0) {
    contextPath = path.join(tmpDir, "context.json");
    await fs.writeFile(contextPath, context, "utf8");
    args.push("--context", contextPath);
  }

  try {
    const { stdout, stderr } = await execFileAsync("cargo", args, {
      cwd: process.cwd(),
      timeout: 5_000,
      maxBuffer: 2 * 1024 * 1024,
    });
    const out = stdout?.toString?.() ?? String(stdout ?? "");
    const err = stderr?.toString?.() ?? String(stderr ?? "");
    const actions: string[] = [];
    out.split("\n").forEach((line) => {
      const m = line.match(/-> EXECUTE: (.+)/);
      if (m) actions.push(m[1].trim());
    });
    return { stdout: out, stderr: err, actions, engine: "rust" as const };
  } catch (err: any) {
    const out = err?.stdout?.toString?.() ?? "";
    const errText = err?.stderr?.toString?.() ?? String(err?.message ?? err);
    const actions: string[] = [];
    (out || errText).split("\n").forEach((line: string) => {
      const m = line.match(/-> EXECUTE: (.+)/);
      if (m) actions.push(m[1].trim());
    });
    return { stdout: out, stderr: errText, actions, engine: "rust" as const, error: true };
  } finally {
    fs.rm(tmpDir, { recursive: true, force: true }).catch(() => {});
  }
}

async function runPythonEvaluator(code: string) {
  const tmpDir = await fs.mkdtemp(path.join(os.tmpdir(), "omnilang-"));
  const filePath = path.join(tmpDir, "input.omni");
  await fs.writeFile(filePath, code, "utf8");
  try {
    const { stdout, stderr } = await execFileAsync("python", [path.join(process.cwd(), "src", "omnilang.py"), filePath], {
      cwd: process.cwd(),
      timeout: 5_000,
      maxBuffer: 2 * 1024 * 1024,
    });
    const out = stdout?.toString?.() ?? String(stdout ?? "");
    const err = stderr?.toString?.() ?? String(stderr ?? "");
    const actions: string[] = [];
    out.split("\n").forEach((line) => {
      const m = line.match(/-> EXECUTE: (.+)/);
      if (m) actions.push(m[1].trim());
    });
    return { stdout: out, stderr: err, actions, engine: "python" as const };
  } catch (err: any) {
    const out = err?.stdout?.toString?.() ?? "";
    const errText = err?.stderr?.toString?.() ?? String(err?.message ?? err);
    const actions: string[] = [];
    (out || errText).split("\n").forEach((line: string) => {
      const m = line.match(/-> EXECUTE: (.+)/);
      if (m) actions.push(m[1].trim());
    });
    return { stdout: out, stderr: errText, actions, engine: "python" as const, error: true };
  } finally {
    fs.rm(tmpDir, { recursive: true, force: true }).catch(() => {});
  }
}

export async function POST(req: Request) {
  const requestId = randomUUID();
  const started = Date.now();
  const clientIp = getClientIp(req);
  const raw = await req.text();

  if (Buffer.byteLength(raw) > MAX_BODY_BYTES) {
    recordRateLimited();
    logJSON({ level: "warn", msg: "validate.payload_too_large", requestId, clientIp });
    return NextResponse.json(
      {
        errors: [{ line: 0, message: "Payload too large." }],
        rules: [],
        actions: [],
        stdout: "",
        stderr: "",
        engine: "none",
        mode: "validator",
        capabilities: [],
        compiler: "not-available",
        requestId,
      },
      { status: 413 },
    );
  }

  if (isRateLimited(clientIp)) {
    recordRateLimited();
    logJSON({ level: "warn", msg: "validate.rate_limited", requestId, clientIp });
    return NextResponse.json(
      {
        errors: [{ line: 0, message: "Rate limit exceeded. Please retry later." }],
        rules: [],
        actions: [],
        stdout: "",
        stderr: "",
        engine: "none",
        mode: "validator",
        capabilities: [],
        compiler: "not-available",
        requestId,
      },
      { status: 429 },
    );
  }

  try {
    const accepted = beginRequest(MAX_INFLIGHT);
    if (!accepted) {
      recordRateLimited();
      logJSON({ level: "warn", msg: "validate.busy", requestId, clientIp });
      return NextResponse.json(
        {
          errors: [{ line: 0, message: "Server busy. Please retry shortly." }],
          rules: [],
          actions: [],
          stdout: "",
          stderr: "",
          engine: "none",
          mode: "validator",
          capabilities: [],
          compiler: "not-available",
          requestId,
        },
        { status: 503 },
      );
    }

    const body = JSON.parse(raw || "{}") as { code?: string; context?: string };
    const code: string = body?.code ?? "";
    const context: string | undefined = body?.context ?? "";

    if (Buffer.byteLength(code) > MAX_CODE_BYTES) {
      recordRateLimited();
      logJSON({ level: "warn", msg: "validate.code_too_large", requestId, clientIp });
      return NextResponse.json(
        {
          errors: [{ line: 0, message: "Policy too large." }],
          rules: [],
          actions: [],
          stdout: "",
          stderr: "",
          engine: "none",
          mode: "validator",
          capabilities: [],
          compiler: "not-available",
          requestId,
        },
        { status: 413 },
      );
    }

    if (Buffer.byteLength(context || "") > MAX_CONTEXT_BYTES) {
      recordRateLimited();
      logJSON({ level: "warn", msg: "validate.context_too_large", requestId, clientIp });
      return NextResponse.json(
        {
          errors: [{ line: 0, message: "Context JSON too large." }],
          rules: [],
          actions: [],
          stdout: "",
          stderr: "",
          engine: "none",
          mode: "validator",
          capabilities: [],
          compiler: "not-available",
          requestId,
        },
        { status: 413 },
      );
    }
    const errors = parseOmniLang(code);
    const rules = extractRules(code);

    // Validate JSON context early to give clear UI feedback
    if (context && context.trim().length > 0) {
      try {
        const parsed = JSON.parse(context);
        const validType = typeof parsed === "object" && parsed !== null;
        if (!validType) {
          throw new Error("Context must be a JSON object or array.");
        }
      } catch (err: any) {
        recordParseError();
        recordRequest(Date.now() - started);
        logJSON({ level: "warn", msg: "validate.invalid_context_json", requestId, clientIp, error: err?.message });
        return NextResponse.json({
          errors: [{ line: 0, message: `Invalid context JSON: ${err?.message ?? err}` }],
          rules,
          actions: [],
          stdout: "",
          stderr: "",
          engine: "none",
          mode: "validator",
          capabilities: [],
          compiler: "not-available",
          requestId,
        });
      }
    }

    // If parse errors exist, return early without executing runtimes
    if (errors.length > 0) {
      recordParseError();
      recordRequest(Date.now() - started);
      logJSON({ level: "info", msg: "validate.parse_errors", requestId, clientIp, errorsCount: errors.length });
      return NextResponse.json({
        errors,
        rules,
        actions: [],
        stdout: "",
        stderr: "",
        engine: "none",
        mode: "validator",
        capabilities: ["parse"],
        compiler: "not-available",
        requestId,
        durationMs: Date.now() - started,
      });
    }

    // Run Rust evaluator; if it fails (e.g., cargo/linker missing), fall back to Python proto
    const rustResult = await runRustEvaluator(code, context);
    const runtime = rustResult.error
      ? DISABLE_PY_FALLBACK
        ? { ...rustResult, engine: "none" as const }
        : await runPythonEvaluator(code)
      : rustResult;

    logJSON({
      level: "info",
      msg: "validate.complete",
      requestId,
      clientIp,
      engine: runtime.engine,
      errorsCount: errors.length,
      rulesCount: rules.length,
      durationMs: Date.now() - started,
      fallback: !!rustResult.error,
    });

    recordRequest(Date.now() - started);

    return NextResponse.json({
      errors,
      rules,
      actions: runtime.actions,
      stdout: runtime.stdout,
      stderr: runtime.stderr,
      engine: runtime.engine,
      mode: "validator",
      capabilities: ["parse", "runtime-eval"],
      compiler: "not-available",
      requestId,
      durationMs: Date.now() - started,
    }, { headers: { "x-request-id": requestId } });
  } catch (err) {
    recordError();
    logJSON({
      level: "error",
      msg: "validate.error",
      requestId,
      clientIp,
      error: err instanceof Error ? err.message : String(err),
    });
    return NextResponse.json(
      {
        errors: [{ line: 0, message: "Internal error during validation." }],
        rules: [],
        actions: [],
        stdout: "",
        stderr: String(err),
        engine: "none",
        mode: "validator",
        capabilities: [],
        compiler: "not-available",
        requestId,
      },
      { status: 500, headers: { "x-request-id": requestId } },
    );
  } finally {
    endRequest();
  }
}

// Internal helper for tests (not exported in production build)
if (process.env.NODE_ENV === "test") {
  const g = globalThis as { __resetRateLimitForTests?: () => void };
  g.__resetRateLimitForTests = () => {
    resetRateBuckets();
    resetTelemetry();
    resetInflight();
  };
}
