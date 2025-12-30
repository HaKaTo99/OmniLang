import { NextResponse } from "next/server";

export async function GET() {
  return NextResponse.json({
    status: "ok",
    uptimeSec: Math.round(process.uptime()),
    timestamp: new Date().toISOString(),
  });
}
