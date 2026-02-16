# OmniLang: High-Assurance Policy Language for Autonomous Systems
*Universal Intent Language (UIL) for the xAetherOS Fabric.*

[![Release](https://img.shields.io/badge/release-v1.2.0--Harmonious-blue.svg)](https://github.com/HaKaTo99/OmniLang/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-Singularity--Ready-success.svg)](docs/ROADMAP.md)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](actions)

> **"Bukan sekadar bahasa pemrograman, melainkan kompas bagi mesin otonom."**

**OmniLang** adalah bahasa spesifikasi kebijakan deklaratif yang dirancang khusus untuk menjembatani antara niat manusia (*Human Intent*) dan eksekusi mesin (*Machine Execution*) pada sistem otonom dan cerdas.

---

## 📋 Daftar Isi
- [Visi & Filosofi](#-visi--filosofi)
- [Arsitektur Sistem](#-arsitektur-sistem)
- [Fitur Utama (v1.1.0)](#-fitur-utama)
- [Struktur Proyek](#-struktur-proyek)
- [Panduan Penggunaan](#-panduan-penggunaan)
- [Dokumentasi Lengkap](#-dokumentasi-lengkap)
- [Roadmap & Status](#-roadmap--status)

---

## 🌟 Visi & Filosofi

### Menghilangkan Kompromi
OmniLang dirancang untuk menggantikan kebutuhan akan banyak bahasa di tumpukan teknologi Anda:

| Bahasa yang Digantikan | Masalah Umum | Solusi OmniLang |
|------------------------|--------------|-----------------|
| **JavaScript / Web** | Kinerja terbatas, runtime berat. | **Full-Stack Universal**. Frontend (Wasm) dan Backend (Native) aman tanpa overhead. |
| **Java / Python** | Berat (JVM), Lambat (GIL), GC Pause. | **Kinerja Native**. Mode `@ownership` menjamin real-time tanpa Garbage Collector. |
| **C++ / Rust** | Tidak aman atau terlalu rumit. | **Kecepatan C++ dengan Ergonomi Pascal**. Aman (*Borrow Checker*) namun *Human-Readable*. |

### Mental Model
- **Seperti Turbo Pascal**: Struktur disiplin ("Canonical Order"), kompilasi native cepat.
- **Seperti SQL**: Deklaratif (INTENT & RULE), fokus pada query konteks (`FOR device IN sensors...`).
- **Seperti React**: Konsep Komponen & State di WebAssembly.

---

## 🏗 Arsitektur Sistem

OmniLang beroperasi dalam dua mode utama yang terintegrasi:

```mermaid
graph TD
    User[Human Intent] -->|Policy .omni| Validator
    User -->|Source Code| Compiler

    subgraph "Core Engine (Declarative)"
        Validator[Validator Runtime] -->|Parses| AST
        Validator -->|Evaluates| Decision[Policy Decision]
    end

    subgraph "Workstation (omc - Imperative)"
        Compiler[Compiler (omc)] -->|Lowers| IR[OmniIR]
        IR -->|Generates| Rust[Rust Native]
        TUI[Cyber IDE] -.-> Compiler
    end

    subgraph "Universal Interface Layer"
        CLI[omc CLI]
        Visual[TUI Dashboard]
        VUI[Voice / VUI]
        NUI[Touch / NUI]
        CUI[Chat / CUI]
        HUI[Serial / HUI]
        GUI[Web Studio]
    end

    Decision --> actuators[System Actuators]
    Rust --> hardware[Native Hardware]
    Visual <--> User
    VUI <--> User
    NUI <--> User
    CUI <--> User
    HUI <--> hardware
    GUI <--> User
```

---

## 💎 Fitur Utama

### 1. High-Assurance Validator (Runtime)
- **Universal Intent Language**: Parser stabil untuk aturan deklaratif dengan *Hand-written Recursive Descent*.
- **Context-Aware**: Evaluasi kebijakan dinamis berdasarkan data JSON eksternal.
- **Rich Stdlib**: Modul bawaan untuk `math`, `crypto`, `time`, dan `tensor` (AI Ops).
- **Observability**: Dukungan Trace ID propagation dan integrasi metrik dasar.

### 2. OmniLang Workstation (Compiler)
- **Dual-Engine Support**: Sinkronisasi real-time antara runtime kebijakan (Core) dan kompiler sistem (`omc`).
- **TUI Dashboard**: IDE terminal canggih dengan visualisasi pipeline kompilasi (Source ➡️ IR ➡️ Output).
- **Cross-Engine Sync**: Menjamin konsistensi logika antara mode deklaratif dan imperatif.

### 3. Universal Access (12 Interfaces)
OmniLang mendukung total 12 kategori interaksi manusia-komputer (HCI) untuk memastikan aksesibilitas universal:

#### 🟢 Kanal Aktif (Siap Pakai)
- **CLI (Command Line)**: Perintah teks standard (CMD, PowerShell, Bash).
- **GUI (Graphical UI)**: Ikon dan jendela (OmniLang Web Studio).
- **TUI (Text-based UI)**: Menu interaktif terminal (Workstation Dashboard).
- **VUI (Voice UI)**: Kendali suara (Web Speech API).
- **NUI (Natural UI)**: Sentuhan dan gesture (Touchscreen).
- **CUI (Conversational UI)**: Percakapan teks/suara (Chatbot AI).
- **HUI (Hardware UI)**: Tombol dan panel fisik (Serial/UART Interface).

#### 🔵 Kanal Futuristik (Roadmap v1.5+)
- **OUI (Organic UI)**: Pemanfaatan layar fleksibel dan permukaan organik.
- **PUI (Perceptual UI)**: Sensor pengenalan wajah, emosi, dan gerakan mata.
- **BUI/BCI (Brain Bridge)**: Interaksi langsung via sinyal otak (Neurotech).
- **MMUI (Multimodal)**: Sinkronisasi beberapa mode sekaligus (Suara+Sentuh+Visi).
- **VR/AR Spatial**: Antarmuka realitas virtual dan augmented (HoloLens/Quest).

---

## 📊 Matriks Interaksi Universal

| Jenis Antarmuka | Cara Interaksi | Contoh di OmniLang |
|-----------------|----------------|-------------------|
| **CLI** | Perintah teks | `omc build main.omni` |
| **GUI** | Grafis (ikon) | OmniLang Studio |
| **TUI** | Terminal Visual | `omc --visual` |
| **VUI** | Suara | 🎤 Voice Command |
| **NUI** | Gesture / Touch | Pinch-zoom & Swipe |
| **CUI** | Chat AI | Interactive AI Assistant |
| **HUI** | Fisik / Serial | `omc --hui` (UART) |
| **OUI** | Layar Fleksibel | 📅 Planned (Flexible Displays) |
| **PUI** | Persepsi Sensor | `@pui` Perception Logic |
| **BUI/BCI** | Sinyal Otak | `--neural` Brain Bridge |
| **MMUI** | Gabungan Mode | Multimodal Orchestration |
| **VR/AR UI**| Spasial 3D | Immersive Coding Space |

---

## 📂 Struktur Proyek

```bash
OmniLang/
├── OmniLang_master_todo.md  # 🗺️ Single Source of Truth Roadmap (Master Plan)
├── README.md                # 📘 Dokumentasi Utama
├── audit_report.md          # 📊 Laporan Status & Verifikasi Stabilitas
├── src/                     # 🧠 Core Engine (Validator/Interpreter Deklaratif)
│   ├── parser.rs            #    - Logika Parsing Intent
│   ├── evaluator.rs         #    - Evaluator Kebijakan & Guard
│   ├── stdlib/              #    - Modul Standard Library (Rust)
│   └── omnilang.py          #    - Fallback Implementation (v1.0 Sync)
├── omc/                     # ⚙️ Workstation (Compiler & IDE Imperatif)
│   ├── src/                 #    - Source code compiler & TUI
│   │   ├── tui_app.rs       #    - Dashboard & Event Loop IDE
│   │   ├── compiler.rs      #    - Pipeline (AST -> IR -> Rust)
│   │   └── hui_serial.rs    #    - Protokol Hardware (HUI)
│   └── Cargo.toml           #    - Dependensi Workstation (Ratatui, dll)
├── examples/                # 💡 Koleksi Kode Contoh (.omni)
└── docs/                    # 📚 Arsip Dokumentasi Mendalam
    ├── ROADMAP.md           #    - Visi Jangka Panjang & Fase G/H
    ├── architecture/        #    - Walkthrough & Implementation Plan
    └── specs/               #    - Spesifikasi Teknis v1.0, v1.1, v1.2
```

---

## 🚀 Panduan Penggunaan (Singularity Edition)

### Prasyarat Sistem

| Software | Versi Minimum | Deskripsi |
|----------|---------------|-----------|
| [Rust](https://www.rust-lang.org/) | 1.70+ | Diperlukan untuk kompilasi `omc` dan Core Engine. |
| [Git](https://git-scm.com/) | 2.x | Untuk manajemen source code. |
| [Python](https://python.org/) | 3.10+ | Diperlukan untuk menjalankan fallback `omnilang.py`. |
| [Node.js](https://nodejs.org/) | 20+ | Untuk menjalankan OmniLang Web Studio (GUI). |

### 1. Instalasi & Persiapan

```bash
# Clone repository
git clone https://github.com/HaKaTo99/OmniLang.git
cd OmniLang

# Build seluruh sistem (omc + core)
cargo build --release
```

### 2. Matriks Perintah Utama

| Perintah | Mode | Kegunaan |
|----------|------|----------|
| `cargo run -- <file.omni>` | **CLI Standard** | Kompilasi file imperatif ke Rust. |
| `cargo run -- <file.omni> --visual` | **TUI IDE** | Workspace interaktif (Edit/IR/Rust/Chat). |
| `cargo run -- exec <file.omni> --context <ctx.json>` | **Core Engine** | Evaluasi kebijakan deklaratif secara langsung. |
| `cargo run -- --hui` | **HUI Serial** | Masuk ke terminal serial (Protokol UART). |
| `cargo run -- --headless <file.omni>` | **IoT Headless** | Eksekusi otomatis tanpa antarmuka visual. |
| `cargo run -- --bci` | **BCI (Experimental)** | Simulasi antarmuka sinyal saraf otak. |
| `cargo run -- --pui` | **PUI (Experimental)** | Antarmuka persepsi sensorik/kamera. |

---

### 1. Kompilasi & IDE Terminal (omc)

**Mode Standard (CLI):**
Mengubah kode OmniLang (`.omni`) menjadi kode Rust asli yang sangat efisien.
```bash
# Generate output.rs
cargo run -- examples/match_demo.omni

# Compile ke binary native
rustc output.rs -o program.exe
./program.exe
```

**Mode Visual (TUI Dashboard):**
Interface lengkap dengan penjelajah file, penyorotan sintaksis, dan panel virtual machine.
```bash
cargo run -- examples/match_demo.omni --visual
```
*Keyboard Shortcuts:*
- `r`: Recompile file
- `1` / `2` / `3` / `4`: Switch Tab (IR / Rust / Logs / Chat)
- `q`: Keluar

---

## 🚀 Panduan Eksekusi 12-Antarmuka

OmniLang dirancang untuk berjalan di mana pun. Berikut adalah langkah-langkah untuk menjalankan masing-masing dari 12 kanal interaksi:

### 🛠️ Kanal Pengembang (Terminal & Compiler)

1. **CLI (Standard)**: Kompilasi kode imperatif ke native Rust.
   ```bash
   cargo run -- examples/loop_demo.omni
   ```
2. **TUI (Cyber IDE)**: Dashboard interaktif dengan real-time view.
   ```bash
   cargo run -- examples/match_demo.omni --visual
   ```
3. **CUI (Chat in TUI)**: Tekan tombol `4` saat berada di mode TUI untuk berinteraksi dengan asisten AI.
4. **HUI (Serial/Hardware)**: Simulasi terminal hardware/serial.
   ```bash
   cargo run -- --hui
   ```
5. **Headless (IoT)**: Eksekusi langsung tanpa antarmuka visual.
   ```bash
   cargo run -- --headless examples/hello.omni
   ```

### 🌐 Kanal Web & Grafis (Web App)
*Pastikan Anda menjalankan Next.js dev server:* `npm run dev`

6. **GUI (Web Studio)**: Buka [localhost:3000](http://localhost:3000) di browser Anda.
7. **VUI (Voice Control)**: Klik ikon mikrofon 🎤 di Web Studio untuk memberikan perintah suara.
8. **NUI (Natural/Touch)**: Gunakan layar sentuh atau trackpad untuk gesture (pinch/swipe) di area visualizer.
9. **CUI (Web Chatbot)**: Gunakan bubble chat di pojok kanan bawah Web Studio.

### 🧪 Kanal Futuristik (Simulator Eksperimental)
*Gunakan flag berikut untuk menjalankan modul riset era v1.5+:*

10. **BCI (Brain Bridge)**: `cargo run -- --bci` (Simulasi sinyal EEG).
11. **PUI (Perceptual)**: `cargo run -- --pui` (Simulasi eye-tracking/face recognition).
12. **OUI (Organic)**: `cargo run -- --organic` (Simulasi haptic/organic feedback).
13. **MMUI (Multimodal)**: `cargo run -- --mmui` (Simulasi sinkronisasi multi-kanal).
14. **VR/AR (Spatial)**: `cargo run -- --vr` (Simulasi mapping ruang 3D).

---

### 2. Runtime Kebijakan (Core Engine)

Digunakan untuk sistem yang membutuhkan pengambilan keputusan dinamis berbasis konteks (JSON) secara deklaratif.
```bash
cargo run -- exec examples/demo.omni --context examples/context.json
```

---

### 3. Hardware & IoT Interaction

OmniLang siap dideploy pada perangkat embedded melalui antarmuka serial atau mode tanpa layar.
```bash
# Mode Serial Interaktif (HUI)
cargo run -- --hui

# Mode Headless untuk Gateway IoT
cargo run -- --headless examples/hello.omni
```


---

## 📚 Dokumentasi Lengkap

Kami telah memfaktorkan dokumentasi agar mudah dinavigasi:

- **Ingin melihat Roadmap?** Buka [OmniLang_master_todo.md](OmniLang_master_todo.md).
- **Ingin sejarah proyek?** Baca [docs/journey/OMNILANG_ORIGINS.md](docs/journey/OMNILANG_ORIGINS.md).
- **Ingin detail grammar?** Baca [docs/specs/SPEC_V1.0.md](docs/specs/SPEC_V1.0.md).

---

## 🗺 Roadmap & Status

| Fase | Fokus Utama | Status | Output |
|------|-------------|--------|--------|
| **v0.9** | Core Engine | ✅ Selesai | Parser, Evaluator, IR |
| **v1.0** | Stability | ✅ Selesai | Zero Warnings, Stdlib |
| **v1.1** | Workstation | ✅ Selesai | TUI Dashboard, Cyber CLI |
| **v1.2** | Harmonious | ✅ Selesai | 7-Interface Sync, Multi-Engine |
| **v2.0** | Singularity | 📅 Planned | BCI, OUI, MMUI, Self-Hosting |

---

## 🤝 Komunitas & Kontribusi

OmniLang dikembangkan oleh **xAetherOS Team** dengan semangat *High-Assurance*.
Silakan berkontribusi melalui Pull Request atau Issue di GitHub.

*Copyright © 2026 xAetherOS Team. Licensed under MIT.*
