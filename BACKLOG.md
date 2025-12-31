# OmniLang 2026 Backlog & Strategic Initiatives

Dokumen ini merinci prioritas pengembangan OmniLang untuk tahun 2026 berdasarkan evaluasi akhir tahun 2025.

---

## 🔝 Prioritas Utama (High Impact)

### 1. [LSP] Implementasi Server LSP (lsp_server.rs)
- **Tujuan**: Menyediakan fitur IDE dasar (Diagnostics, Hover, Completion).
- **Detail**: Melengkapi `src/lsp_server.rs` agar terintegrasi dengan `Parser` dan `Checker`.
- **Estimasi**: 5 hari

### 2. [Security] Secure RNG dan Implementasi AES-256
- **Tujuan**: Menjamin keamanan data dan privasi dalam kebijakan.
- **Detail**: Integrasi CSPRNG dan implementasi cipher AES-256-GCM ke modul `crypto`.
- **Estimasi**: 8 hari

### 3. [Network] Integrasi Driver HTTP (std::net/http)
- **Tujuan**: Membuka akses ke API eksternal dan cloud services.
- **Detail**: Implementasi client/server HTTP asinkron di Stdlib.
- **Estimasi**: 6 hari

---

## 🛠️ Pengembangan Bahasa & Engine

### 4. [Borrow-Checker] Stress Tests & Fuzzing
- **Tujuan**: Menjamin stabilitas manajemen memori pada skenario kompleks.
- **Detail**: Membuat harness fuzzing untuk `checker.rs`.
- **Estimasi**: 7 hari

### 5. [Type-System] Support Generics (Syntax & Inference)
- **Tujuan**: Meningkatkan ekspresi bahasa dan reusabilitas kode.
- **Detail**: Update AST dan Logic Unifier untuk mendukung tipe generik.
- **Estimasi**: 12 hari

### 6. [Type-System] Union Types & MATCH Exhaustiveness
- **Tujuan**: Penanganan tipe data yang lebih fleksibel dan aman.
- **Detail**: Implementasi Tagged Unions dan validasi kelengkapan sekat `MATCH`.
- **Estimasi**: 8 hari

---

## 🤖 Integrasi AI & Ekosistem Otonom

### 7. [Robotics] ROS2 Runtime Adapter (Prototype)
- **Tujuan**: OmniLang sebagai bahasa kebijakan utama di robotika.
- **Detail**: Integrasi dengan RMW (ROS Middleware) atau HTTP Bridge.
- **Estimasi**: 10 hari

### 8. [Model] Tensor Ops: Convolution & Activation
- **Tujuan**: Mendukung primitif Neural Network di Stdlib.
- **Detail**: Optimasi `stdlib/tensor.rs` untuk operasi ML dasar.
- **Estimasi**: 10 hari

---

## 🖥️ Tooling & DX (Developer Experience)

### 9. [Studio] Visual Debugger MVP
- **Tujuan**: Memudahkan debugging kebijakan yang kompleks melalui UI.
- **Detail**: Backend kordinasi breakpoint dan state inspection.
- **Estimasi**: 14 hari

### 10. [Network] Implementasi Protokol MQTT (std::net/mqtt)
- **Tujuan**: Konektivitas IoT di lapisan Edge.
- **Estimasi**: 6 hari

### 11. [Security] Asymmetric Crypto (RSA/ECDSA)
- **Tujuan**: Tanda tangan digital dan verifikasi identitas aktor.
- **Estimasi**: 7 hari

### 12. [CI/CD] Nightly Fuzzing & Static Analysis
- **Tujuan**: Otomatisasi deteksi bug di tingkat engine.
- **Estimasi**: 2 hari

---

## 📅 Roadmap 2026
- **Q1**: Focus on LSP & Developer Experience.
- **Q2**: Security hardening & Networking foundations.
- **Q3**: Robotics & AI/Tensor extensions.
- **Q4**: Visual tooling & Ecosystem pilots.
