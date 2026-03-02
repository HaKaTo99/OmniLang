# QUICKSTART (v2.2.0)

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

### Contoh Eksekusi OODA Loop (Mesh AI & Hardware Actuator)
Jalankan tiga terminal terpisah untuk menguji orkestrasi Jaringan Mesh End-to-End:
```bash
# Terminal 1: Nyalakan Worker Node berbekal Neural Network ONNX & Capability Token
cargo run -- serve examples/ooda_loop/ai_worker.omni --port 8081 --token "ooda-2026"

# Terminal 2: Nyalakan Actuator Node (HUI) yang menghubungkan port COM3 ke aktuator fisik
cargo run -- serve examples/ooda_loop/actuator.omni --port 8082 --token "ooda-2026" --hui COM3

# Terminal 3: Sensor Node yang meng-intercept deteksi dan memicu Rantai Eksekusi Jarak Jauh
cargo run -- test examples/ooda_loop/sensor.omni
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
