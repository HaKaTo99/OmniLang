# Getting Started OmniLang v1.2.0 (Harmonious Singularity)

Dokumen ini merangkum langkah cepat untuk menjalankan OmniLang di berbagai kanal antarmuka, mencakup mode Deklaratif (Core) dan Imperatif (omc).

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

## Cara Menjalankan

### 1. Standard CLI (Imperatif/omc)
```bash
cargo run -- examples/loop_demo.omni
```

### 2. TUI Visual IDE
```bash
cargo run -- examples/match_demo.omni --visual
```

### 3. Core Engine (Deklaratif)
```bash
cargo run -- exec examples/demo.omni --context examples/context.json
```

### 4. Hardware/Serial (HUI)
```bash
cargo run -- --hui      # Serial terminal mode
cargo run -- --headless examples/hello.omni # Headless mode
```

### 5. Futuristic Interfaces (Experimental)
```bash
cargo run -- --bci      # Brain-Computer Interface
cargo run -- --pui      # Perceptual Interface
```

## Kompilasi lintas platform
OmniLang mendukung target Rust Native, WASM, dan roadmap menuju JVM (Android) serta C++ (Legacy Mobile).
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
