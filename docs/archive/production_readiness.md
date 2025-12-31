# Production Readiness Plan

## 1) Scope & Feature Target
- Target MVP produksi: runtime eksekusi + policy enforcement sesuai SPEC_V1.0.
- Komponen wajib: parser, checker, evaluator, runtime (deterministik, sandbox), stdlib dasar (I/O terbatas, math, time), konfigurasi fitur (flag) dan matriks OS/arch yang didukung.
- Kebijakan kompatibilitas: tentukan versi minimal Node, Rust, Python bila digunakan dalam pipeline integrasi.

## 2) Hardening Eksekusi
- Sandbox evaluator (pembatas CPU/mem/waktu) dan isolasi konteks input (deny-by-default terhadap fungsi berbahaya).
- Validasi input: size limit kode/konteks, schema validation JSON, deteksi recursion/loop tak terbatas pada policy.
- Rate limiting & throttling untuk endpoint validate; observasi error bursts.
- Jalur gagal aman (fail-safe) dan pesan error tidak membocorkan detail internal.

## 3) CI/CD & Quality Gates
- Lint/format + unit/integration + e2e web (Next.js) + property/fuzz tests (parser/evaluator).
- Security: dependency audit (npm audit, cargo audit), SAST lint (tsc strict + clippy -D warnings), supply-chain pinning (lockfiles).
- Build artefak release (tag/versioning) dan publish hanya dari main/protected branch.

## 4) Test Coverage & Regression Safety
- Perluas test suite untuk seluruh 18 contoh .omni sebagai golden tests.
- Tambah property tests untuk parser/evaluator (units, loops, IN, dot-path).
- Tambah load tests untuk /api/validate.

## 5) Observability
- Logging terstruktur (request ID, policy ID), metrik latency/p99/error rate, health checks.
- Tracing: span untuk parse → check → eval.

## 6) Performance & Load
- Profil jalur panas (lex/parse/eval) dan optimasi alloc/clone di Rust.
- Uji beban dengan variasi ukuran policy/konteks; target SLA latency; dokumentasi tunable (worker pool, cache, timeouts).

## 7) Operasional & Keamanan
- Runbook insiden dan rollback plan; backup/publish artefak; versioned configs.
- Secrets management (bukan di repo); konfigurasi env per stage; boundary izin minimal.

## 8) Next Actions (prioritas)
1. Aktifkan CI lint/test (Rust + Next.js) dan audit deps. **[DONE]**
2. Tambah guard input (size limit + schema) dan rate limiting di /api/validate. **[DONE]**
3. Siapkan suite golden test untuk 18 contoh .omni. **[DONE via tests/test_examples_batch.py]**
4. Tambah observability dasar (structured log + request ID) di API + endpoint health/metrics + JSON log pipeline. **[DONE]**
5. Tambah API tests guard rails (/api/validate) dengan node:test (size/rate-limit/context). **[DONE]**
6. Batasan inflight concurrency dan opsi matikan fallback Python via env. **[DONE]**
7. Readiness endpoint dengan inflight check + build version. **[DONE]**
8. Prometheus-friendly metrics endpoint (/api/metrics-prom) dan contoh manifest K8s dengan probes/limits. **[DONE]**
