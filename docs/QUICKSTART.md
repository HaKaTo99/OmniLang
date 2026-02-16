# QUICKSTART (v1.2.0-Harmonious)

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

## Menjalankan OmniLang (12-Interface Guide)

### 1. Kanal Pengembang (Terminal)
- **CLI Standard**: `cargo run -- examples/loop_demo.omni`
- **TUI Visual IDE**: `cargo run -- examples/match_demo.omni --visual`
- **HUI Serial**: `cargo run -- --hui`
- **Headless Mode**: `cargo run -- --headless examples/hello.omni`

### 2. Kanal Web & Grafis (Next.js)
*Server: `npm run dev`*
- **GUI (Web Studio)**: [localhost:3000](http://localhost:3000)
- **VUI (Voice Control)**: Klik ikon 🎤 di Web Studio untuk perintah suara.
- **NUI (Touch/Gesture)**: Interaksi sentuh/gesture di area visualizer.
- **CUI (Chat Assistant)**: Bubble chat di Web Studio atau Tab 4 di TUI.

### 3. Kanal Futuristik (Simulation)
- **BCI (Brain)**: `cargo run -- --bci`
- **PUI (Perceptual)**: `cargo run -- --pui`
- **OUI (Organic)**: `cargo run -- --organic`
- **MMUI (Multimodal)**: `cargo run -- --mmui`
- **VR/AR (Spatial)**: `cargo run -- --vr`


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
*Rilis v1.2.0-Harmonious mendukung Dual-Engine (Declarative + Imperative) dan 12 jenis antarmuka interaksi universal.*
