# QUICKSTART

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
# Jalankan contoh loop demo
cargo run -- examples/loop_demo.omni
```

Output akan menampilkan eksekusi kebijakan termasuk loop `FOR` dan `WHILE` dengan guard keamanan.

## Pengujian
```bash
# Unit + property‑based tests
cargo test

# Benchmark (compile‑only, lihat latency)
cargo bench --no-run
```

## CI/CD
Pipeline CI otomatis dijalankan pada setiap push/pull‑request (lihat `.github/workflows/ci.yml`).

---
*Dokumen ini memberikan langkah‑langkah cepat untuk memulai pengembangan dengan OmniLang.*
