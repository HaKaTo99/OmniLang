# OmniLang: High-Assurance Policy Language for Autonomous Systems
*Universal Intent Language (UIL) for the xAetherOS Fabric.*

[![Release](https://img.shields.io/badge/release-v2.1.0-blue.svg)](https://github.com/HaKaTo99/OmniLang/releases/tag/v2.1.0)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-Distributed_Intelligence-success.svg)](docs/release_v2.1.0_draft.md)

> **"Satu Bahasa, Satu Fabric. AI, Blockchain, & IoT dalam Harmoni."**

**OmniLang** adalah bahasa spesifikasi kebijakan deklaratif yang dirancang khusus untuk menjembatani antara niat manusia (*Human Intent*) dan eksekusi mesin (*Machine Execution*) pada sistem otonom dan cerdas dalam ekosistem xAetherOS.

---

## 📋 Navigasi Cepat
- **[Visi & Filosofi](#-visi--filosofi)**: Mengapa OmniLang ada.
- **[Quickstart](docs/QUICKSTART.md)**: Instalasi dan cara menjalankan dalam 5 menit.
- **[Roadmap](docs/ROADMAP.md)**: Status pengembangan dan rencana masa depan.
- **[Interfaces Guide](docs/guides/INTERFACES.md)**: Panduan lengkap 12 antarmuka universal (CLI, TUI, GUI, BCI, dll).
- **[Language Specs](docs/spec/INDEX.md)**: Spesifikasi teknis sintaksis, tipe data, dan keamanan.

---

## 🌟 Visi & Filosofi

### Menghilangkan Kompromi
OmniLang menggantikan kebutuhan akan banyak bahasa di tumpukan teknologi Anda dengan satu solusi universal yang aman, cepat, dan mudah dibaca.

| Masalah Umum | Solusi OmniLang |
|--------------|-----------------|
| Runtime berat / Lambat | **Kinerja Native**. Tanpa GC, dioptimalkan untuk real-time. |
| Tidak aman / Rumit | **Borrow Checker Safety**. Keamanan Rust dengan ergonomi Pascal. |
| Fragmentasi Antarmuka | **Universal Access**. Satu logika untuk semua kanal interaksi. |

---

## 🏗 Arsitektur System: Dual-Engine

OmniLang beroperasi dengan dua mesin eksekusi yang saling melengkapi:
1. **Core Engine (Declarative)**: Evaluasi kebijakan dinamis (`INTENT`, `RULE`, `POLICY`) secara real-time.
2. **Workstation Compiler (Imperatif)**: Kompilasi logika sistem (`fn`, `match`, `let`) menjadi Rust/Native.

### Sorotan Fitur v2.1.0
- **Native AI Inference (`@oracle`)**: Terintegrasi seketika dengan **ONNX Runtime Engine** tanpa C++ bindings yang rumit. [Baca Tutorial 10 Menit ke ONNX](docs/tutorials/onnx_10min.md).
- **Secure Distributed Mesh (`@mesh`)**: Lakukan RPC Mesh antara node Sensor, Worker, dan Aktuator dengan jaminan TCP *Capability Tokens*. [Panduan Membangun App Terdistribusi](docs/tutorials/distributed_mesh.md).

---

## 📂 Struktur Proyek

```bash
OmniLang/
├── docs/                    # 📚 Dokumentasi Lengkap (Roadmap, Spec, Guides)
├── src/                     # 🧠 Core Engine (Validator Rust & Web Studio)
├── omc/                     # ⚙️ Workstation (Compiler & TUI IDE)
├── examples/                # 💡 Koleksi Kode Contoh (.omni)
├── tests/                   # 🧪 Suite Pengujian (Rust & Python)
└── README.md                # 📘 Pintu Masuk Visi
```

---

## 🤝 Kontribusi

Silakan pelajari [CONTRIBUTING.md](docs/CONTRIBUTING.md) untuk mulai membangun masa depan otonom bersama kami.

*Copyright © 2026 xAetherOS Team. Licensed under MIT.*
