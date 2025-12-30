# 🚀 Release Notes: OmniLang v1.0 

Selamat datang di era **OmniLang v1.0 (Universal Intent Language)**! Rilis ini menandai transformasi besar dari bahasa pemrograman imperatif menjadi bahasa deklarasi kebijakan berbasis niat (*Intent-based*) yang kuat, aman, dan dapat dibaca manusia maupun mesin.

## 🌟 Fitur Utama

### 1. Arsitektur Intent-First
Format dokumen kebijakan kini mengikuti *Canonical Order* yang sangat terstruktur:
- **INTENT**: Deklarasi tujuan utama.
- **ACTOR**: Definisi entitas primer dan sekunder.
- **CONTEXT**: Domain, Lokasi, dan Fase operasional.
- **ASSUMPTION**: (Baru) Definisi asumsi lingkungan sebelum aturan dijalankan.
- **RULE**: Mendukung logika kondisional (`IF/THEN`) dan loop (`FOR/WHILE`).
- **CONSTRAINT**: Batasan Legal, Etis, dan Teknis.
- **IMPACT**: Analisis manfaat, risiko, dan trade-off.
- **TRACE**: Referensi regulasi dan bukti moral.
- **REVIEW**: (Baru) Jadwal tinjauan kebijakan otomatis.

### 2. Paritas Mesin (Rust & Python)
- **Rust Core (Primary)**: Engine berkinerja tinggi dengan keamanan memori dan pengujian properti yang ketat.
- **Python Fallback (Sync)**: Kami menjamin sinkronisasi 1:1 antara mesin Rust dan Python untuk memastikan integrasi OmniLang Studio tetap berjalan di berbagai lingkungan server.

### 3. Keamanan Runtime
- Implementasi Safety Guards: `MAX_LOOP_ITERATIONS` dan `MAX_LOOP_TIME_MS` untuk mencegah loop tak terbatas pada kebijakan kompleks.
- Dukungan Unit: Penanganan otomatis angka dengan unit (contoh: `10kmh`, `25C`, `1m`).
- Konversi persentase ke fraksi (`20%` → `0.2`) dan normalisasi jarak/waktu (km/m/cm/mm, s/ms/h) pada evaluator.

### 4. Ruang Lingkup v1.0 (Validator + Compiler Runner)
- Fokus rilis: validasi intent dan logika kebijakan (parser + evaluator Rust dengan fallback Python) **serta** generator runner native/Wasm berbasis IR.
- Tersedia compiler ke binary/Wasm: `cargo run -- compile <file.omni> --target native|wasm --out <path>`.
- Standard library: `time`, `io`, `web`, `math`, `string`, `collections`, `json` kini ditambah `crypto` (SHA-256, HMAC-SHA256, Base64) dan `tensor` (dot, matmul kecil) serta helper zona waktu/truncation.
- Runtime scheduler/konkurensi (`OmniRoutine`) ada untuk eksekusi aksi paralel terbatas.
- Observability: `format_log` menyertakan timestamp, level, dan trace id untuk korelasi log end-to-end.

## 🛠 Perbaikan Teknis
- Memperbaiki masalah linker `link.exe` pada environment Windows melalui integrasi MSVC.
- Menambahkan koleksi pengujian `parser_examples` untuk menjamin validitas semua file `.omni`.
- Membersihkan redundansi kode dan peringatan `dead_code` untuk kompilasi yang lebih bersih.

## 📁 Apa yang Baru di Folder Repo?
- `/docs`: Berisi laporan audit mendalam, walkthrough migrasi, dan rencana implementasi.
- `/examples`: Koleksi contoh kebijakan mulai dari Drone, Pabrik, Smart City, hingga showcase lintas domain (AI Ethics, Zero Trust, AML, Sustainability, Supply Chain).
- `.github/workflows`: Automasi CI lintas stack dan lintas platform.

## 🤖 CI/CD
- Rust (Linux): `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --verbose`, `cargo bench --no-run`, `cargo audit`, `cargo deny` dengan cache registry/target.
- Python (Linux): `python -m unittest discover -s tests` pada Python 3.10 dan 3.11.
- Frontend (Linux): `npm ci` dan `npm run lint` dengan Node 20.

---
*OmniLang v1.0: "Write Intent, Ensure Compliance."*
