import { NextResponse } from "next/server";
import { getTelemetry } from "@/lib/telemetry";

export async function GET() {
  const snapshot = getTelemetry();
  const avgLatencyMs = snapshot.totalRequests > 0 ? snapshot.totalDurationMs / snapshot.totalRequests : 0;
  return NextResponse.json({
    ...snapshot,
    avgLatencyMs,
    timestamp: new Date().toISOString(),
  });
}
