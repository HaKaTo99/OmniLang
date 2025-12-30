type LogLevel = "debug" | "info" | "warn" | "error";

interface LogPayload {
  level: LogLevel;
  msg: string;
  requestId?: string;
  clientIp?: string;
  engine?: string;
  durationMs?: number;
  fallback?: boolean;
  errorsCount?: number;
  rulesCount?: number;
  error?: string;
}

const LOG_LEVEL = (process.env.LOG_LEVEL ?? "info").toLowerCase();
const levelOrder: Record<LogLevel, number> = { debug: 10, info: 20, warn: 30, error: 40 };

function shouldLog(level: LogLevel): boolean {
  return levelOrder[level] >= levelOrder[LOG_LEVEL as LogLevel] ?? 20;
}

export function logJSON(payload: LogPayload) {
  if (!shouldLog(payload.level)) return;
  // Avoid cyclic/undefined; stringify compactly
  try {
    console.log(JSON.stringify(payload));
  } catch (err) {
    console.error("logJSON_failed", err);
  }
}
