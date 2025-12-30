# OmniLang: High-Assurance Policy Language for Autonomous Systems

**OmniLang** adalah bahasa spesifikasi kebijakan deklaratif yang dirancang khusus untuk menjembatani antara niat manusia (*Human Intent*) dan eksekusi mesin (*Machine Execution*) pada sistem otonom dan cerdas.

> **"Bukan sekadar bahasa pemrograman, melainkan kompas bagi mesin otonom."**

> **Catatan scope v1.x:** fokus utama sebagai **validator intent** (parser + evaluator) dengan compiler runner IR/native/Wasm, stdlib fungsional, dan observability terintegrasi.

## Visi: Menghilangkan Kompromi

OmniLang dirancang untuk menggantikan kebutuhan akan banyak bahasa di tumpukan teknologi Anda:

| Bahasa yang Digantikan | Kompromi yang Diatasi | Solusi OmniLang |
| :--- | :--- | :--- |
| **JavaScript / React / Angular** | Kinerja terbatas, terikat pada runtime JS yang besar. | **Full-Stack Universal.** Frontend (Wasm) dan Backend (Native) dalam satu bahasa, aman, dan tanpa overhead JS/V8. |
| **Java / Python / Go** | Berat (JVM), Lambat (GIL), atau GC Pause tak terduga. | **Kinerja Native & Kontrol Memori.** Kompilasi ke *native binary* ringan. Mode `@ownership` menjamin real-time tanpa Garbage Collector (GC). |
| **C++ / Rust** | C++ tidak aman; Rust memiliki kurva belajar curam. | **Kecepatan C++ dengan Ergonomi Pascal.** Jaminan keamanan memori (Borrow Checker) namun dengan sintaksis yang mudah dibaca manusia (*Human-Readable*). |

## Mental Model: Analogi OmniLang

Jika Anda familiar dengan teknologi berikut, inilah cara memahami OmniLang:

1.  **Seperti Turbo Pascal / Delphi**:
    *   Mengutamakan **Struktur & Ketertiban** (*Canonical Order*).
    *   Kompilasi **Native yang Sangat Cepat**.
    *   Hasil akhir berupa binary tunggal yang **Mandiri** (tidak butuh dependencies runtime).

2.  **Seperti SQL**:
    *   Bersifat **Deklaratif**. Anda menuliskan `INTENT` (Niat) dan `RULE` (Aturan), biarkan *engine* yang mengurus optimasi eksekusi.
    *   Fokus pada **Filter & Query** konteks (misal: `FOR device IN sensors WHERE ...`).

3.  **Seperti Go (Golang)**:
    *   **Kesederhanaan** di atas segalanya. Tidak ada fitur "magic" yang tersembunyi.
    *   Tooling modern yang terintegrasi (Formatter, Linter, Test) yang cepat.

4.  **Seperti Angular / React**:
    *   Mendukung konsep **Komponen & State**, tetapi dijalankan di **WebAssembly (Wasm)** untuk performa native di browser, melampaui batas kecepatan JavaScript.

## Warisan & Evolusi (Heritage)

OmniLang bukanlah penciptaan ulang roda, melainkan kulminasi dari evolusi bahasa pemrograman selama 70 tahun terakhir:

### Generasi Awal (Fondasi Logika)
*   **Assembly & Fortran**: OmniLang mewarisi efisiensi tingkat rendah (via Rust/LLVM) dan ketepatan komputasi.
*   **Lisp & COBOL**: Fokus pada *Intent* (Kecerdasan) seperti Lisp, dan keterbacaan bisnis (*Business Readable*) seperti COBOL.
*   **Pascal & ALGOL**: Mengadopsi struktur yang disiplin dan "Canonical Order".

### Era Pertengahan (Sistem & Objek)
*   **Ada**: Inspirasi utama untuk **Safety-Critical Systems** (sistem militer/medis). OmniLang adalah "Ada untuk era AI".
*   **SQL**: Sifat deklaratif untuk query data/konteks (`SELECT/FOR`, `WHERE/IF`).
*   **Java & C++**: Mengambil performa C++ tetapi membuang *Garbage Collector* (GC) yang tidak terduga milik Java.

### Era Modern (Efisiensi & Tooling)
*   **Rust**: OmniLang dibangun di atas bahu raksasa ini, meminjam **Borrow Checker** untuk keamanan memori tanpa kompromi.
*   **Go & Python**: Mengutamakan kesederhanaan (Go) dan keterbacaan (Python), menghapus sintaksis yang rumit (*boilerplate*).
*   **TypeScript**: Sistem tipe yang kuat untuk mencegah error sebelum runtime.

**Kesimpulan:** OmniLang adalah langkah selanjutnya—**Era Intent (2025+)**—di mana kita tidak lagi sekadar memberi instruksi *step-by-step* ke mesin, melainkan mendefinisikan *Batasan (Constraints)* dan *Tujuan (Goals)*.

## Posisi dalam Ekosistem: Dirigen & Spesialis

Bagaimana OmniLang berinteraksi dengan ribuan bahasa yang sudah ada? OmniLang tidak mencoba menggantikan peran mereka sebagai *general purpose language*, melainkan bertindak sebagai **governance layer** (lapisan tata kelola).

| Kategori Bahasa | Contoh | Hubungan dengan OmniLang |
| :--- | :--- | :--- |
| **System & Low-Level** | Assembly, C, C++, Rust | **Memanfaatkan (Leverage).** OmniLang menggunakan teknologi mereka (Rust/LLVM) untuk menghasilkan binary yang secepat C dan seaman Rust. |
| **General Purpose** | Java, Python, Go, C# | **Mengatur (Govern).** Bahasa-bahasa ini membangun *mekanisme* (misal: driver motor), sementara OmniLang mendefinisikan *kebijakan* (misal: "Jangan berputar jika panas > 50°C"). |
| **Scripting & Web** | JS, TS, PHP, Ruby | **Mengamankan (Guard).** OmniLang dapat dikompilasi ke WebAssembly (Wasm) untuk berjalan di browser/server sebagai "penjaga" yang memvalidasi input logika berisiko. |
| **Logic & Business** | SQL, COBOL, Excel | **Modernisasi (Elevate).** Menggantikan ribuan baris logika bisnis yang tersebar ("spaghetti code") menjadi satu dokumen `.omni` yang terpusat dan mudah diaudit. |

> **Analogi:** Jika bahasa lain adalah *Peralatan Tukang* (Palu, Gergaji, Obeng), maka OmniLang adalah *Cetak Biru Arsitek* (Blueprint) yang memastikan gedung dibangun sesuai spesifikasi dan tidak roboh.

## Fitur Inti yang Sudah Divalidasi

1. **Parser & Evaluator:** Universal Intent Language dengan RULE/LOOP (FOR/WHILE), IN operator, path bertitik + indeks.
2. **Compiler Runner:** Emit IR JSON, native runner, dan wasm32-wasi runner berbasis interpreter IR.
3. **Stdlib:** math, string (regex), time (UTC/truncation), collections, json path, io, web (file://), crypto (SHA-256/HMAC/Base64), tensor (dot/matmul).
4. **Observability & Metrics:** `format_log` (timestamp/level/trace), trace id propagation, Prometheus/OpenMetrics export.
5. **Action ABI & Scheduler:** `ActionResult` untuk hasil eksekusi aksi, OmniRoutine untuk eksekusi paralel terbatas.

## Status Saat Ini (Scope v1.0)

OmniLang v1.x berperan sebagai **validator intent** yang stabil (parser, evaluator Rust + fallback Python) dengan compiler runner native/Wasm, stdlib kaya (termasuk crypto/tensor), metrics, dan observability. Eksekusi end-to-end melalui runner IR tersedia.

**Status rilis saat ini (v1.0 dengan penambahan terbaru):**
- Core: parser/evaluator Universal Intent Language, loops FOR/WHILE, IN operator, path bertitik.
- Compiler: target IR JSON, native runner, wasm32-wasi runner (smoke-tested).
- Stdlib: 9 modul (math, string+regex, time dengan UTC/truncation, collections, json path, io, web file://, crypto SHA-256/HMAC/Base64, tensor dot/matmul).
- Observability: `format_log` menyertakan timestamp/level/trace id; trace-aware CLI/runtime output.
- Metrics: Prometheus/OpenMetrics export aktif.
- Action ABI & Scheduler: `ActionResult` dengan elapsed, OmniRoutine untuk eksekusi paralel terbatas.

**Belum Selesai (roadmap ringkas):**
* IDE/LSP + VS Code extension.
* Perf: cache IR/runner, optional JIT/bytecode flattening.
* Stdlib lanjutan: crypto nonce/random, tensor ops tambahan.
* Observability sink: file/OTLP exporter, action-level timing logs.
* Integrasi ekosistem: ROS2/Kubernetes/IoT.

**Cara berkontribusi cepat:**
* Tambah contoh kebijakan dan konteks di folder examples/ (lintas domain/industri).
* Tambah integrasi atau tool seputar validator (CLI, editor extension, pipeline CI) tanpa menyentuh compiler terlebih dahulu.
* Jalankan regresi sebelum PR: `cargo test --all` untuk Rust, `npm test` untuk guard rails API/validator di frontend.

## Sorotan Evaluator v1.0

- Operator boolean berantai: OR/AND/NOT pada satu baris kondisi.
- Dukungan `IN` pada array literal atau referensi; fallback ke perbandingan skalar jika bukan array (mis. `Mode IN 2`).
- Path bertitik dengan indeks array (`sensor.flags[0]`) dan literal array JSON di sisi kanan.
- Loop `FOR` mengikat iterator ke elemen array nyata sehingga ekspresi seperti `device.status` valid.
- Contoh siap jalan: `cargo run -- exec examples/evaluator_features.omni --context examples/evaluator_features_context.json`.

## Cara Memulai dan Berkontribusi

### 1. **Prasyarat**
Proyek inti ditulis dalam Rust. Anda perlu menginstal [Rust toolchain](https://www.rust-lang.org/tools/install).

### 2. **Kloning Repositori**
```bash
git clone https://github.com/HaKaTo99/OmniLang.git
cd OmniLang
```

### Eksekusi (validator runtime)
```bash
cargo run -- exec examples/demo.omni --context examples/context.json
```
Menghasilkan log keputusan, aksi yang terpicu, dan status guard.

### Kompilasi (IR / native / wasm)
```bash
# IR JSON
cargo run -- compile examples/demo.omni --out target/demo_ir.json

# Native runner (embed IR + interpreter)
cargo run -- compile examples/demo.omni --target native --out target/demo_native.bin

# Wasm runner (wasm32-wasi, exported evaluate/get_output)
cargo run -- compile examples/demo.omni --target wasm --out target/demo_wasm.wasm
```
Catatan: gunakan path file langsung tanpa tanda [] atau link markdown. Artefak wasm dieksekusi dengan wasmtime: `wasmtime target/demo_wasm.wasm --invoke evaluate <ctx_ptr> <ctx_len>` kemudian baca hasil via `get_output_ptr/len` dari linear memory.

**Tip:** pasang target `wasm32-wasi` terlebih dahulu (`rustup target add wasm32-wasi`). Uji asap wasm di `cargo test --all` akan otomatis dilewati bila target belum tersedia, tetapi lebih baik memasangnya agar jalur wasm tetap terjaga.

### Stdlib core (v1.2)
- `stdlib::time`: `now_iso8601()`, `now_unix_millis()`
- `stdlib::io`: `read_json_file(path)`, `write_json_pretty(path, &value)`
- `stdlib::web`: `get_json("file:///abs/path.json")` (saat ini hanya `file://` untuk menjaga determinisme tanpa jaringan)

Semua fungsi stdlib kini mengembalikan `Result<_, OmniError>` agar penanganan kesalahan lebih terstruktur (Io/Json/Network/InvalidInput/Unsupported).

### OmniRoutine (scheduler ringan)
- `OmniRoutine::new(max_parallel)` untuk membatasi jumlah tugas paralel.
- `run(tasks, worker)` mengeksekusi `RoutineTask { name, payload }` dengan worker closure, menjaga urutan hasil sesuai input dan mengembalikan `RoutineResult { name, output: Result<Value, OmniError> }`.
- Runtime kini menyediakan `execute_actions_with_routine(actions, max_parallel, worker)` agar eksekusi aksi pasca-keputusan dapat dijalankan paralel secara aman dengan konteks ter-clone.

Contoh singkat (Rust):

```rust
use omnilang_core::stdlib::{now_iso8601, read_json_file};

let ts = now_iso8601();
let data = read_json_file("examples/context.json")?;
println!("{} -> keys: {}", ts, data.as_object().map(|o| o.len()).unwrap_or(0));
```
