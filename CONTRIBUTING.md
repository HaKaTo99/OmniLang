# Panduan Kontribusi untuk OmniLang

Kami sangat antusias dengan minat Anda untuk berkontribusi pada OmniLang! Karena proyek ini berada di Fase Prototipe Inti, kontribusi pada arsitektur sangat berharga.

Harap patuhi panduan berikut untuk menjaga kualitas kode dan alur kerja yang efisien.

## 🤝 Kode Etik

Untuk memastikan lingkungan yang ramah, semua kontributor harus mematuhi [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/) (Akan ditambahkan ke file `CODE_OF_CONDUCT.md` di masa mendatang).

## 💡 Bagaimana Cara Berkontribusi?

Ada beberapa cara utama untuk membantu proyek ini saat ini:

### 1. Mengajukan Isu (Issues)

* **Bug Report:** Jika Anda menemukan kesalahan pada *Parser* atau kegagalan pada *Test Suite* **Borrow Checker**, ajukan sebagai isu dengan label `bug`. Berikan langkah-langkah reproduksi yang jelas.
* **Feature Request:** Jika Anda memiliki ide untuk penyempurnaan sintaks atau fitur inti (misalnya pada desain `OmniRoutine`), ajukan sebagai isu dengan label `enhancement` atau `design`.

### 2. Mengajukan *Pull Request* (PR)

Kami menerima PR yang fokus pada penyempurnaan dan implementasi fitur yang tercantum dalam *Roadmap*.

#### Alur Kerja PR

1.  **Fork** Repositori OmniLang.
2.  **Clone** hasil *fork* Anda ke mesin lokal.
3.  Buat *branch* baru (contoh: `git checkout -b feature/implement-runtime-scheduler`).
    * Gunakan format: `fix/nama-perbaikan` atau `feat/nama-fitur`.
4.  Lakukan perubahan dan pastikan semua *test suite* (`cargo test`) **LULUS**.
5.  **Commit** perubahan Anda. Gunakan pesan *commit* yang deskriptif dan mengikuti konvensi **Conventional Commits** (misalnya: `feat: Menambahkan scheduler dasar ke runtime`).
6.  *Push* *branch* baru Anda ke *fork* Anda di GitHub.
7.  Buat **Pull Request (PR)** ke *branch* `main` di repositori utama OmniLang.

#### Standar Kode

* **Pemformatan:** Semua kode Rust harus diformat menggunakan `rust fmt`.
* **Komentar:** Komentar harus jelas, terutama untuk bagian arsitektur kritis (Borrow Checker, Type Checker).
* **Test:** Semua fitur dan perbaikan harus disertai dengan *unit tests* yang memadai.

Terima kasih telah membantu membangun masa depan pemrograman dengan OmniLang!
