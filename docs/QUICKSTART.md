# QUICKSTART (v1.2.1)

Panduan cepat untuk memulai pengembangan dengan OmniLang.

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

## Cara Menjalankan
OmniLang mendukung 12 antarmuka universal. Untuk detail lengkap setiap kanal, lihat **[Interfaces Guide](guides/INTERFACES.md)**.

### Contoh Eksekusi Dasar (CLI)
```bash
cargo run -- examples/loop_demo.omni
```

### Visual IDE (TUI)
```bash
cargo run -- examples/match_demo.omni --visual
```

### Web Studio (GUI)
1. Jalankan server: `npm run dev`
2. Buka: [localhost:3000](http://localhost:3000)

---

## Pengujian
```bash
# Unit + property‑based tests (Rust)
cargo test

# Benchmark (pemeriksaan performa)
cargo bench --no-run

# Pengujian fallback Python
python -m unittest discover -s tests
```

---
*OmniLang: Satu Bahasa, Segala Antarmuka. Untuk roadmap pengembangan, lihat [ROADMAP.md](ROADMAP.md).*
