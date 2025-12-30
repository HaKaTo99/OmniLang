export type TelemetrySnapshot = {
  totalRequests: number;
  totalErrors: number;
  totalRateLimited: number;
  totalParseErrors: number;
  totalDurationMs: number;
  lastRequestAt?: number;
  lastErrorAt?: number;
};

const counters: TelemetrySnapshot = {
  totalRequests: 0,
  totalErrors: 0,
  totalRateLimited: 0,
  totalParseErrors: 0,
  totalDurationMs: 0,
};

export function recordRequest(durationMs: number) {
  counters.totalRequests += 1;
  counters.totalDurationMs += durationMs;
  counters.lastRequestAt = Date.now();
}

export function recordError() {
  counters.totalErrors += 1;
  counters.lastErrorAt = Date.now();
}

export function recordRateLimited() {
  counters.totalRateLimited += 1;
}

export function recordParseError() {
  counters.totalParseErrors += 1;
}

export function getTelemetry(): TelemetrySnapshot {
  return { ...counters };
}

export function resetTelemetry() {
  counters.totalRequests = 0;
  counters.totalErrors = 0;
  counters.totalRateLimited = 0;
  counters.totalParseErrors = 0;
  counters.totalDurationMs = 0;
  counters.lastRequestAt = undefined;
  counters.lastErrorAt = undefined;
}
