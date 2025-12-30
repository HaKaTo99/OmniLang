require("ts-node/register/transpile-only");
const assert = require("node:assert/strict");
const { test, beforeEach } = require("node:test");

process.env.VALIDATE_BODY_MAX = "2000";
process.env.VALIDATE_CODE_MAX = "500";
process.env.VALIDATE_CONTEXT_MAX = "500";
process.env.VALIDATE_RATE_LIMIT_MAX = "2";
process.env.VALIDATE_RATE_LIMIT_WINDOW_MS = "60000";
process.env.NODE_ENV = "test";

const { POST, __resetRateLimitForTests } = require("../src/app/api/validate/route.ts");

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

beforeEach(() => {
  __resetRateLimitForTests();
});

test("returns 413 for oversized policy", async () => {
  const largeCode = "X".repeat(600);
  const req = new Request("http://localhost/api/validate", {
    method: "POST",
    body: JSON.stringify({ code: largeCode, context: "" }),
  });
  const res = await POST(req);
  assert.equal(res.status, 413);
  const body = await res.json();
  assert.match(body.errors[0].message, /too large/i);
});

test("returns invalid context error without hitting runtime", async () => {
  const req = new Request("http://localhost/api/validate", {
    method: "POST",
    body: JSON.stringify({ code: VALID_CODE, context: "not-json" }),
  });
  const res = await POST(req);
  assert.equal(res.status, 200);
  const body = await res.json();
  assert.equal(body.engine, "none");
  assert.ok(body.errors.length > 0);
  assert.match(body.errors[0].message, /Invalid context JSON/i);
});

test("rate limiting returns 429 after limit", async () => {
  const badCode = "RULE:\n- THEN OOPS";
  const reqFactory = () =>
    new Request("http://localhost/api/validate", {
      method: "POST",
      body: JSON.stringify({ code: badCode, context: "" }),
    });

  const res1 = await POST(reqFactory());
  assert.equal(res1.status, 200);
  const res2 = await POST(reqFactory());
  assert.equal(res2.status, 200);
  const res3 = await POST(reqFactory());
  assert.equal(res3.status, 429);
  const body3 = await res3.json();
  assert.match(body3.errors[0].message, /Rate limit/i);
});
