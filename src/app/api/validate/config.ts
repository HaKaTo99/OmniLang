export const MAX_BODY_BYTES = parseInt(process.env.VALIDATE_BODY_MAX ?? "200000", 10); // ~200 KB request guard
export const MAX_CODE_BYTES = parseInt(process.env.VALIDATE_CODE_MAX ?? "100000", 10); // ~100 KB policy limit
export const MAX_CONTEXT_BYTES = parseInt(process.env.VALIDATE_CONTEXT_MAX ?? "100000", 10); // ~100 KB context limit
export const RATE_LIMIT_WINDOW_MS = parseInt(process.env.VALIDATE_RATE_LIMIT_WINDOW_MS ?? "60000", 10); // 1 minute
export const RATE_LIMIT_MAX_REQUESTS = parseInt(process.env.VALIDATE_RATE_LIMIT_MAX ?? "30", 10); // per window per ip
export const MAX_INFLIGHT = parseInt(process.env.VALIDATE_MAX_INFLIGHT ?? "4", 10);
export const DISABLE_PY_FALLBACK = (process.env.DISABLE_PY_FALLBACK ?? "false").toLowerCase() === "true";
export const BUILD_VERSION = process.env.BUILD_VERSION ?? process.env.VERCEL_GIT_COMMIT_SHA ?? "unknown";
