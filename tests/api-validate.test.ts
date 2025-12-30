import assert from "node:assert/strict";
import { test, beforeEach } from "node:test";

// Set tight limits for tests to avoid heavy runtime work
process.env.VALIDATE_BODY_MAX = "2000";
process.env.VALIDATE_CODE_MAX = "500";
process.env.VALIDATE_CONTEXT_MAX = "500";
process.env.VALIDATE_RATE_LIMIT_MAX = "2";
process.env.VALIDATE_RATE_LIMIT_WINDOW_MS = "60000";
process.env.NODE_ENV = "test";

const modulePromise = import("../src/app/api/validate/route");
let POST: any;
let resetRateLimitForTests: any;

const VALID_CODE = `INTENT: Demo
ACTOR:
- Primary: Tester
CONTEXT:
- Domain: Testing
RULE:
- IF Temp > 30 THEN Cool
CONSTRAINT:
- Technical: OK
IMPACT:
- Benefit: Safety
TRACE:
- Evidence: Logs
`;

beforeEach(async () => {
  if (!POST) {
    const mod = await modulePromise;
    POST = mod.POST;
    resetRateLimitForTests = (globalThis as any).__resetRateLimitForTests;
  }
  if (resetRateLimitForTests) resetRateLimitForTests();
});

test("returns 413 for oversized policy", async () => {
  const largeCode = "X".repeat(600); // above VALIDATE_CODE_MAX
  const req = new Request("http://localhost/api/validate", {
    method: "POST",
    body: JSON.stringify({ code: largeCode, context: "" }),
  });
  const res = await POST(req as any);
  assert.equal(res.status, 413);
  const body = await res.json();
  assert.match(body.errors[0].message, /too large/i);
});

test("returns invalid context error without hitting runtime", async () => {
  const req = new Request("http://localhost/api/validate", {
    method: "POST",
    body: JSON.stringify({ code: VALID_CODE, context: "not-json" }),
  });
  const res = await POST(req as any);
  assert.equal(res.status, 200);
  const body = await res.json();
  assert.equal(body.engine, "none");
  assert.ok(body.errors.length > 0);
  assert.match(body.errors[0].message, /Invalid context JSON/i);
});

test("rate limiting returns 429 after limit", async () => {
  const badCode = "RULE:\n- THEN OOPS"; // parse errors => no runtime
  const reqFactory = () =>
    new Request("http://localhost/api/validate", {
      method: "POST",
      body: JSON.stringify({ code: badCode, context: "" }),
    });

  const res1 = await POST(reqFactory() as any);
  assert.equal(res1.status, 200);
  const res2 = await POST(reqFactory() as any);
  assert.equal(res2.status, 200);
  const res3 = await POST(reqFactory() as any);
  assert.equal(res3.status, 429);
  const body3 = await res3.json();
  assert.match(body3.errors[0].message, /Rate limit/i);
});
