# Panduan Kontribusi untuk OmniLang

Kami sangat antusias dengan minat Anda untuk berkontribusi pada OmniLang! Karena proyek ini berada di Fase Prototipe Inti, kontribusi pada arsitektur sangat berharga.

Harap patuhi panduan berikut untuk menjaga kualitas kode dan alur kerja yang efisien.

##  Kode Etik

Untuk memastikan lingkungan yang ramah, semua kontributor harus mematuhi kode etik yang telah kami tetapkan (lihat [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)).

##  Bagaimana Cara Berkontribusi?

Ada beberapa cara utama untuk membantu proyek ini saat ini:

### 1. Mengajukan Isu (Issues)

* **Bug Report:** Jika Anda menemukan kesalahan pada *Parser* atau kegagalan pada *Test Suite* **Borrow Checker**, ajukan sebagai isu dengan label `bug`. Berikan langkah-langkah reproduksi yang jelas.
* **Feature Request:** Jika Anda memiliki ide untuk penyempurnaan sintaks atau fitur inti (misalnya pada desain `OmniRoutine`), ajukan sebagai isu dengan label `enhancement` atau `design`.

### 2. Mengajukan *Pull Request* (PR)

Kami menerima PR yang fokus pada penyempurnaan dan implementasi fitur yang tercantum dalam *Roadmap*.

#### Alur Kerja PR

1.  **Fork** Repositori OmniLang.
2.  **Clone** hasil *fork* Anda ke mesin lokal.
3.  Buat *branch* baru (contoh: `git checkout -b feat/validator-improvement`).
    * Gunakan format: `fix/nama-perbaikan` atau `feat/nama-fitur`.
4.  Lakukan perubahan.
5.  **Jalankan regresi cepat:**
    * Rust: `cargo test --all`
    * Frontend/API validator: `npm test`
6.  **Commit** perubahan Anda. Gunakan pesan *commit* yang deskriptif dan mengikuti konvensi **Conventional Commits** (misalnya: `feat: tambah konversi unit di evaluator`).
7.  *Push* *branch* baru Anda ke *fork* Anda di GitHub.
8.  Buat **Pull Request (PR)** ke *branch* `main` di repositori utama OmniLang.

#### Checklist Cepat (selaras scope v2.1.0 - Distributed Intelligence)

- Fokus pada penguatan fabric: Mesh (`src/mesh`), AI (`src/onnx_oracle.rs`), atau keamanan (`security.rs`) selain validator inti.
- Tambahkan/ubah contoh di `examples/` jika ada sintaks atau fitur baru yang disentuh.
- Tambahkan tes: Rust `tests/` atau API guard rails `tests/api-validate.test.ts` jika relevan.
- Dokumentasi singkat: perbarui README atau docs/ sesuai perubahan perilaku.
- Format: `cargo fmt` untuk Rust, `npm run lint` jika Anda menyentuh frontend.

#### Standar Kode

* **Pemformatan:** Semua kode Rust harus diformat menggunakan `rust fmt`.
* **Komentar:** Komentar harus jelas, terutama untuk bagian arsitektur kritis (Borrow Checker, Type Checker).
* **Test:** Semua fitur dan perbaikan harus disertai dengan *unit tests* yang memadai.

## Area yang Membutuhkan Bantuan

Bagi Anda yang ahli di spesifikasi eksternal, kami mengundang kontribusi untuk:
- Integrasi Backend WASM.
- Pengembangan TUI dan integrasi GUI.
- Template contoh aplikasi _Real-World_ untuk arsitektur IoT.

## 🔥 _Good First Issues_

Belum familiar dengan kompilator Rust? Tim inti rutin membuat label `good first issue` di GitHub untuk memandu pemula masuk ke proyek OmniLang, dengan topik seputar:
- Menambah persentase _test coverage_.
- Memperbaiki redaksi *error message* yang membingungkan.
- Meluruskan dan menerjemahkan dokumen.
- Membantu konfigurasi *pipeline* GitHub Actions.

## Komunikasi & Diskusi (Community Hub)

Bergabunglah bersama *Brain-Trust* xAetherOS kami!
* **GitHub Discussions**: Untuk tanya jawab, gagasan fitur, dan dukungan asinkronus (Silakan tabrak tab `Discussions` di repositori kami).
* *(Tautan Discord/Telegram resmi akan segera menyusul)*.

Terima kasih telah membantu membangun masa depan pemrograman dengan OmniLang!
