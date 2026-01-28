# Getting Started OmniLang v1.0 (Intent Validator)

Dokumen ini merangkum langkah cepat untuk memvalidasi berkas `.omni` dengan engine Rust (primer) dan Python (fallback) yang sudah selaras dengan spesifikasi v1.0.

## Prasyarat
- Rust toolchain (stable) dan `cargo`.
- Python 3.10+ (untuk fallback dan pengujian Python).
- Node 20 + `npm ci` bila ingin menjalankan OmniLang Studio (opsional).

## Instalasi
```bash
git clone https://github.com/HaKaTo99/OmniLang.git
cd OmniLang
```

## Validasi cepat (Rust)
```bash
cargo test
```
Menjalankan parser, evaluator, property tests, dan contoh `.omni` termasuk loop guard.

## Menjalankan contoh
```bash
# Rust path
cargo run -- examples/loop_demo.omni

# Evaluator showcase (IN, array literal, dot path, loop iterator binding)
cargo run -- exec examples/evaluator_features.omni --context examples/evaluator_features_context.json

# Python fallback (setara v1.0)
python src/omnilang.py examples/hello.omni
```

## Pengujian Python
```bash
python -m unittest discover -s tests
```

## Kompilasi ke Native/Wasm
```bash
# IR (default)
cargo run -- compile examples/loop_demo.omni --target ir --out target/policy_ir.json

# Native binary runner (embeds IR)
cargo run -- compile examples/loop_demo.omni --target native --out target/omnilang_native_runner.exe

# WASM (wasi) runner
cargo run -- compile examples/loop_demo.omni --target wasm --out target/omnilang_wasm_runner.wasm
```
Hasil native/Wasm adalah runner kecil yang memuat IR dan mengeksekusi via interpreter IR.

## Struktur Penting
- `src/omnilang.py` : Fallback interpreter v1.0
- `src/parser.rs`, `src/evaluator.rs` : Engine Rust
- `examples/*.omni` : Contoh berurutan *Canonical Order* (INTENT→ACTOR→CONTEXT→[ASSUMPTION]→RULE→CONSTRAINT→IMPACT→TRACE→[REVIEW])
- `SPEC_V1.0.md` : Rujukan grammar resmi

## Catatan
- Kompilasi native/Wasm sudah tersedia via perintah di atas.
- Stdlib waktu: `to_utc_iso8601`, `truncate_to_date_iso8601`, `truncate_to_hour_iso8601`, `now_*`, `add_millis`, `duration_between_ms`, `parse_iso8601`.
- Stdlib crypto: `hash_sha256`, `hmac_sha256`, `base64_encode`, `base64_decode`.
- Stdlib tensor: `dot`, `matmul` untuk operasi vektor/matriks kecil.
- Logging: `format_log` kini menyertakan timestamp, level, dan trace id untuk setiap output CLI/runtime.
