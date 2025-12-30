import { NextResponse } from "next/server";
import { getTelemetry } from "@/lib/telemetry";

export async function GET() {
  const t = getTelemetry();
  const avgLatencyMs = t.totalRequests > 0 ? t.totalDurationMs / t.totalRequests : 0;
  const lines = [
    `# HELP omnilang_requests_total Total requests processed`,
    `# TYPE omnilang_requests_total counter`,
    `omnilang_requests_total ${t.totalRequests}`,
    `# HELP omnilang_errors_total Total error responses`,
    `# TYPE omnilang_errors_total counter`,
    `omnilang_errors_total ${t.totalErrors}`,
    `# HELP omnilang_rate_limited_total Total rate limited requests`,
    `# TYPE omnilang_rate_limited_total counter`,
    `omnilang_rate_limited_total ${t.totalRateLimited}`,
    `# HELP omnilang_parse_errors_total Total parse errors detected`,
    `# TYPE omnilang_parse_errors_total counter`,
    `omnilang_parse_errors_total ${t.totalParseErrors}`,
    `# HELP omnilang_avg_latency_ms Average latency in milliseconds`,
    `# TYPE omnilang_avg_latency_ms gauge`,
    `omnilang_avg_latency_ms ${avgLatencyMs}`,
  ];
  return new NextResponse(lines.join("\n") + "\n", {
    status: 200,
    headers: { "Content-Type": "text/plain; version=0.0.4" },
  });
}
