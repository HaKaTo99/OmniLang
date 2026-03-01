# QUICKSTART (v2.1.0)

Panduan cepat untuk memulai pengembangan dengan OmniLang.

## Prasyarat
- Rust toolchain (stable) dengan `cargo` terinstal.
- Git untuk meng‑clone repositori.

## Instalasi
```bash
# Clone repository
git clone https://github.com/HaKaTo99/OmniLang.git
cd OmniLang

# Build eksekusi
cargo build
```

## Cara Menjalankan
OmniLang mendukung 12 antarmuka universal. Untuk detail lengkap setiap kanal, lihat **[Interfaces Guide](guides/INTERFACES.md)**.

### Contoh Eksekusi Distributed Mesh & AI (OODA Loop)
Jalankan dua terminal terpisah untuk menguji orkestrasi Jaringan Mesh:
```bash
# Terminal 1: Nyalakan Worker Node berbekal Neural Network ONNX & Capability Token
cargo run -- serve examples/multi_node_worker.omni --port 8081 --token "dummy-token"

# Terminal 2: Sensor Node yang meng-intercept eksekusi fungsional jarak jauh
cargo run -- test examples/multi_node_sensor.omni
```

### Eksekusi Rutin Dasar (CLI)
```bash
cargo run -- test examples/loop_demo.omni
```

### Visual IDE (TUI)
```bash
cargo run -- examples/match_demo.omni --visual
```

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
