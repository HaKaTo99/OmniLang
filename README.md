# OmniLang
Bahasa Pemrograman

# OmniLang: The Universal, Zero-Compromise Programming Language

**Status: Prototipe Inti (Proof of Concept)**
OmniLang adalah bahasa pemrograman yang dirancang untuk menghilangkan kompromi performa, keamanan, dan universalitas yang ada pada bahasa modern. Kami menargetkan kinerja Native (C++) dengan jaminan keamanan memori (Rust) untuk aplikasi Full-Stack, Real-Time, dan High Concurrency.

## Visi: Menghilangkan Kompromi

OmniLang dirancang untuk menggantikan kebutuhan akan banyak bahasa di tumpukan teknologi Anda:

| Bahasa yang Digantikan | Kompromi yang Diatasi | Solusi OmniLang |
| :--- | :--- | :--- |
| **JavaScript/React** | Kinerja terbatas, terikat pada runtime JS. | **Full-Stack Universal.** Frontend (Wasm) dan Backend (Native) dalam satu bahasa, aman, dan tanpa overhead JS. |
| **Java/Python** | Berat (JVM), Lambat (GIL), GC Pause tak terduga. | **Kinerja Native & Kontrol Memori.** Kompilasi ke *native binary* ringan. Mode `@ownership` menjamin real-time tanpa GC. |
| **C++** | Sangat Tidak Aman, Risiko Kebocoran Memori & Data Race. | **Kecepatan C++ dengan Keamanan Rust.** Jaminan keamanan memori melalui **Borrow Checker** yang diimplementasikan di Rust. |

## Fitur Inti yang Sudah Divalidasi (Implemented in Rust PoC)

Saat ini, kami berfokus pada validasi inti keamanan dan performa:

1.  **Parser Inti:** Mampu memahami sintaks OmniLang.
2.  **Type Checker:** Validasi tipe pada waktu kompilasi.
3.  **Borrow Checker:** Mekanisme keamanan memori yang ketat, terinspirasi Rust, untuk memastikan tidak ada *data race* atau *null pointer* tanpa bergantung pada *Garbage Collector* (GC).

## Status Saat Ini

Saat ini, proyek berada dalam **Fase Prototipe Inti**. Kami telah membuktikan bahwa arsitektur keamanan dimungkinkan.

**Apa yang Belum Selesai:**
* **Compiler Fungsional Penuh:** Belum dapat menghasilkan kode yang dapat dieksekusi (*native* atau Wasm) dari sintaks OmniLang secara keseluruhan.
* **Standard Library:** `std::web`, `std::tensor`, dan modul I/O lainnya.
* **Runtime:** Implementasi *Scheduler* dan Model Konkurensi (`OmniRoutine`).

## Cara Memulai dan Berkontribusi

### 1. **Prasyarat**
Proyek inti ditulis dalam Rust. Anda perlu menginstal [Rust toolchain](https://www.rust-lang.org/tools/install).

### 2. **Kloning Repositori**
```bash
git clone [https://github.com/HaKaTo99/OmniLang.git](https://github.com/HaKaTo99/OmniLang.git)
cd OmniLang
