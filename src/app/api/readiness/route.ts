import { NextResponse } from "next/server";
import { getTelemetry } from "@/lib/telemetry";
import { getInflight } from "../validate/inflight-store";
import { MAX_INFLIGHT, BUILD_VERSION } from "../validate/config";

export async function GET() {
  const inflight = getInflight();
  const telemetry = getTelemetry();
  const healthy = inflight < MAX_INFLIGHT;
  return NextResponse.json(
    {
      status: healthy ? "ok" : "degraded",
      inflight,
      maxInflight: MAX_INFLIGHT,
      totalRequests: telemetry.totalRequests,
      totalErrors: telemetry.totalErrors,
      buildVersion: BUILD_VERSION,
      timestamp: new Date().toISOString(),
    },
    { status: healthy ? 200 : 503 },
  );
}
