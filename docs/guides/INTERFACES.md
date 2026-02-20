# OmniLang Universal Interfaces Guide

OmniLang dirancang dengan visi **Universal Access**, mendukung total 12 kategori interaksi manusia-komputer (HCI) untuk memastikan aksesibilitas di berbagai skenario, dari terminal pengembang hingga simulator neural.

## 🟢 Kanal Aktif (Siap Pakai)

### 1. CLI (Command Line Interface)
Antarmuka standar untuk pengoperasian compiler dan eksekusi skrip melalui terminal (CMD, PowerShell, Bash).
- **Perintah**: `cargo run -- <file.omni>`
- **Guna**: Kompilasi imperatif ke native Rust.

### 2. TUI (Text-based UI / Cyber IDE)
Dashboard terminal interaktif untuk pengembangan yang lebih visual tanpa meninggalkan shell.
- **Perintah**: `cargo run -- <file.omni> --visual`
- **Shortcut**: `1`/`2`/`3`/`4` (Tab), `r` (Recompile), `q` (Quit).

### 3. GUI (Graphical UI / Web Studio)
IDE berbasis web modern yang menyediakan editor kode, visualizer runtime, dan manajemen kebijakan.
- **Akses**: [localhost:3000](http://localhost:3000) (Jalankan `npm run dev`).

### 4. VUI (Voice UI)
Kendali suara untuk sistem otonom menggunakan Web Speech API.
- **Cara**: Klik ikon mikrofon 🎤 di Web Studio.

### 5. NUI (Natural UI)
Interaksi alami melalui sentuhan, gesture (pinch/swipe), dan drag-drop.
- **Cara**: Gunakan layar sentuh atau trackpad di area visualizer Web Studio.

### 6. CUI (Conversational UI / Chatbot)
Interaksi berbasis percakapan teks atau suara dengan asisten AI.
- **Akses**: Bubble chat di Web Studio atau Tab 4 di TUI.

### 7. HUI (Hardware UI / Serial)
Antarmuka langsung ke perangkat monitor fisik melalui protokol serial/UART.
- **Perintah**: `cargo run -- --hui`

---

## 🔵 Kanal Futuristik (Simulation/Roadmap v1.5+)

Kanal-kanal berikut saat ini berjalan dalam mode simulas/eksperimental untuk riset xAetherOS.

| Jenis | Flag Perintah | Keterangan |
|-------|---------------|------------|
| **BCI** | `--bci` | Brain-Computer Interface (Simulasi EEG). |
| **PUI** | `--pui` | Perceptual UI (Eye-tracking / Face Recognition). |
| **OUI** | `--oui` | Organic UI (Layar fleksibel / haptic). |
| **MMUI**| `--mmui`| Multimodal (Sinkronisasi multi-kanal). |
| **VR/AR**| `--vr`  | Spatial Interface (3D Space Mapping). |

---

## 🚀 Panduan Eksekusi Headless
Untuk sistem IoT atau gateway yang tidak memerlukan antarmuka visual sama sekali:
```bash
cargo run -- --headless examples/hello.omni
```

---
*OmniLang: Satu Bahasa, Segala Antarmuka.*
