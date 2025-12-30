# QUICKSTART (Validator v1.0)

## Prasyarat
- Rust toolchain (stable) dengan `cargo` terinstal.
- Git untuk meng‑clone repositori.

## Instalasi
```bash
# Clone repository
git clone https://github.com/HaKaTo99/OmniLang.git
cd OmniLang

# Build (debug)
cargo build
```

## Menjalankan contoh
```bash
# Jalankan contoh loop demo (Rust engine)
cargo run -- examples/loop_demo.omni

# Jalankan via Python fallback (sinkron v1.0)
python src/omnilang.py examples/hello.omni

# Eksekusi dengan runtime (aksi + guard)
cargo run -- exec examples/demo.omni --context examples/context.json  # context optional

# Contoh konteks lain (runtime exec)
cargo run -- exec examples/demo.omni --context examples/context_smartcity.json
cargo run -- exec examples/demo.omni --context examples/context_fintech.json
```

Output akan menampilkan eksekusi kebijakan termasuk loop `FOR` dan `WHILE` dengan guard keamanan.

## Pengujian
```bash
# Unit + property‑based tests (Rust)
cargo test

# Benchmark (compile‑only, lihat latency)
cargo bench --no-run

# Pengujian fallback Python
python -m unittest discover -s tests
```

## CI/CD
Pipeline CI otomatis dijalankan pada setiap push/pull‑request (lihat `.github/workflows/ci.yml`).

---
*Rilis v1.0 berfokus pada validasi intent (parser/evaluator). Compiler, stdlib, dan runtime eksekusi belum tersedia.*
