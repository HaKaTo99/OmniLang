# OmniLang: High-Assurance Policy Language for Autonomous Systems
*Universal Intent Language (UIL) for the xAetherOS Fabric.*

[![Release](https://img.shields.io/badge/release-v1.1.0-blue.svg)](https://github.com/HaKaTo99/OmniLang/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-production--ready-success.svg)](OmniLang_master_todo.md)
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

    subgraph "Core Engine (Root)"
        Validator[Validator Runtime] -->|Parses| AST
        Validator -->|Evaluates| Decision[Policy Decision]
        Stdlib[Standard Library] -.-> Validator
    end

    subgraph "Workstation (omc)"
        Compiler[Compiler (omc)] -->|Lowers| IR[OmniIR]
        IR -->|Generates| Rust[Rust Native]
        IR -->|Generates| Wasm[WebAssembly]
        TUI[Cyber Dashboard] -.-> Compiler
    end

    Decision --> actuators[System Actuators]
    Rust --> hardware[Native Hardware]
    Wasm --> browser[Browser / Edge]
```

---

## 💎 Fitur Utama

### 1. High-Assurance Validator (Runtime)
- **Universal Intent Language**: Parser stabil untuk aturan deklaratif.
- **Context-Aware**: Evaluasi kebijakan berdasarkan data JSON eksternal.
- **Rich Stdlib**: `math`, `crypto` (SHA-256), `time` (UTC), `tensor` (AI Ops).
- **Observability**: Trace ID propagation dan Prometheus metrics export.

### 2. OmniLang Workstation (Compiler)
- **Cyber CLI**: Antarmuka terminal modern ("Military Grade").
- **TUI Dashboard**: Visualisasi real-time proses kompilasi (Source ➡️ IR ➡️ Output).
- **Zero Warnings**: Codebase yang diaudit ketat untuk stabilitas maksimal.

---

## 📂 Struktur Proyek

```bash
OmniLang/
├── OmniLang_master_todo.md  # 🗺️ Single Source of Truth Roadmap
├── README.md                # 📘 Dokumentasi Utama (Anda di sini)
├── audit_report.md          # 📊 Laporan Status Proyek
├── src/                     # 🧠 Core Engine (Validator/Interpreter)
│   ├── parser.rs            #    - Logic Parsing
│   ├── evaluator.rs         #    - Logic Evaluasi
│   ├── stdlib/              #    - Standard Library Modules
│   └── omnilang.py          #    - Fallback Python Implementation
├── omc/                     # ⚙️ Workstation (Compiler & IDE)
│   ├── src/                 #    - Source code compiler
│   │   ├── tui_app.rs       #    - TUI Dashboard Logic
│   │   └── compiler.rs      #    - Compilation Pipeline
│   └── Cargo.toml           #    - Config untuk 'omc'
├── examples/                # 💡 Contoh Kode Kebijakan (.omni)
└── docs/                    # 📚 Arsip Dokumentasi
    ├── journey/             #    - Narasi & Sejarah
    ├── specs/               #    - Spesifikasi Teknis
    └── guides/              #    - Tutorial
```

---

## 🚀 Panduan Penggunaan

### 1. Instalasi
Pastikan Anda memiliki [Rust](https://www.rust-lang.org/) terinstal.
```bash
git clone https://github.com/HaKaTo99/OmniLang.git
cd OmniLang
```

### 2. Menjalankan Validator (Runtime)
Digunakan untuk mengevaluasi kebijakan/policy secara langsung.
```bash
# Skenario: Evaluasi demo.omni dengan konteks data
cargo run -- exec examples/demo.omni --context examples/context.json
```

### 3. Menggunakan Workstation (compiler)
Digunakan untuk pengembangan, visualisasi, dan kompilasi ke binary.
```bash
cd omc
cargo run -- demo.omni --visual
```
*Tip: Gunakan flag `--visual` untuk membuka TUI Dashboard.*

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
| **v0.1-v0.9** | Core Engine | ✅ Selesai | Parser, Evaluator, IR |
| **v1.0** | Stability | ✅ Selesai | Zero Warnings, Stdlib |
| **v1.1** | Workstation | ✅ Selesai | TUI Dashboard, Cyber CLI |
| **v1.2** | Functional | 📅 Planned | Pattern Matching, Lambda |
| **v2.0** | Ecosystem | 📅 Planned | Self-Hosting, Package Manager |

---

## 🤝 Komunitas & Kontribusi

OmniLang dikembangkan oleh **xAetherOS Team** dengan semangat *High-Assurance*.
Silakan berkontribusi melalui Pull Request atau Issue di GitHub.

*Copyright © 2026 xAetherOS Team. Licensed under MIT.*
