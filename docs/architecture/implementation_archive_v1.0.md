# Implementasi OmniLang v1.0: Migrasi & Peningkatan Parser

Tujuan dari rencana ini adalah untuk membawa OmniLang sepenuhnya ke spesifikasi v1.0 ("Universal Intent Language") dengan memigrasikan file legacy dan menambahkan dukungan penuh untuk seksi `ASSUMPTION` dan `REVIEW`.

## Proposed Changes

### v1.1 Compiler Dasar (draft)

**Tujuan:** menghasilkan artefak eksekusi minimal (Wasm dan/atau Native) dari AST yang sudah distabilkan, tanpa belum menyentuh stdlib atau runtime konkurensi.

**Scope inti:**
- Frontend: gunakan parser/checker yang ada untuk memproduksi IR `PolicyIR` (JSON) yang stabil.
- IR: **implemented** — normalisasi `Policy` -> `PolicyIR { sections: canonical order, rules: [RuleIR], flat_rules, constraints, impacts, guard metadata }` dengan flatten loop body dan anotasi guard.
- Backend Wasm: **stub WAT tersedia** (embed IR di data section, ekspor `evaluate` stub). **Next:** ganti stub dengan wasm32-wasi biner yang memanggil evaluator.
- Backend Native: re-use evaluator Rust; sediakan `evaluate_ir(policy_ir: &str, context: &str)` dengan ABI sama untuk parity.

**Langkah implementasi:**
1) Tambah crate/module `ir` untuk builder dari AST ke IR serializable; sertakan `serde` untuk JSON.
2) Tambah target `wasm32-wasi`: `cargo build -p omnilang --target wasm32-wasi` (atau workspace `omnilang_core` split jika perlu).
3) CLI eksperimen: `cargo run -- compile examples/demo.omni --target wasm --out target/demo_wasm_stub.wat` (stub WAT sudah ada; ganti dengan wasm32-wasi ketika siap codegen).
4) Validasi: uji snapshot IR dari `examples/*.omni` agar deterministik; uji eksekusi evaluator terhadap IR (bukan AST) untuk menjaga parity.
5) Dokumentasi: tambahkan diagram alir parse -> IR -> evaluate (Wasm/Native) dan daftar keterbatasan (tidak ada stdlib, tidak ada sandboxing).

**Out-of-scope v1.1:** stdlib (`std::web`, `std::tensor`), scheduler/OmniRoutine, host I/O, keamanan sandbox CPU/ulimit (ditandai untuk v2.x).

### Core Engine (Rust Backend)

#### [MODIFY] [ast.rs](file:///d:/GitHub/OmniLang/src/ast.rs)
- Menambahkan field `assumptions: Vec<String>` dan `reviews: Vec<Review>` ke struct `Policy`.
- Menambahkan struct `Review` untuk menyimpan data interval dan kriteria tinjauan.

#### [MODIFY] [parser.rs](file:///d:/GitHub/OmniLang/src/parser.rs)
- Mengimplementasikan `parse_assumptions` untuk menangani seksi `ASSUMPTION:`.
- Mengimplementasikan `parse_reviews` untuk menangani seksi `REVIEW:`.
- Memperbarui `parse_policy` agar mengenali dan mengisi field baru tersebut.

---

### Examples & Documentation

#### [MODIFY] [hello.omni](file:///d:/GitHub/OmniLang/examples/hello.omni)
- Mengubah format dari gaya pemrograman imperatif (mod/func) ke format deklaratif Intent.

#### [MODIFY] [demo.omni](file:///d:/GitHub/OmniLang/examples/demo.omni)
- Mengubah format ke Intent v1.0 yang lebih kompleks (Drone/Robot scenario).

### [Component Environment]
Summary: Menyiapkan toolchain Rust agar dapat melakukan kompilasi dan pengujian secara lokal.

#### [SETUP] Microsoft Visual Studio Build Tools
- Menginstal MSVC, Windows SDK, dan VCTools via `winget`.
- Memastikan `link.exe` tersedia untuk `cargo build` dan `cargo test`.

---

### [Component Python Backend]
Summary: Migrasi interpreter Python (fallback) ke v1.0 agar selaras dengan spesifikasi OmniLang v1.0.

#### [MODIFY] [omnilang.py](file:///d:/GitHub/OmniLang/src/omnilang.py)
- Memperbarui Lexer untuk mendukung keyword v1.0 (INTENT, ACTOR, dll).
- Memperbarui Parser untuk mendukung struktur Canonical Order.
- Memperbarui Evaluator untuk menyimulasikan eksekusi Intent v1.0.

#### [MODIFY] [test_python_pipeline.py](file:///d:/GitHub/OmniLang/tests/test_python_pipeline.py)
- Memperbarui unit test agar menggunakan sintaks v1.0.

---

### [Component Verification]
Summary: Menjalankan suite pengujian lengkap pada Rust dan Python.

#### [RUN] Cargo Test
- Menjalankan `cargo test parser_examples parser_units_loops unit_numbers` setelah environment siap.

---

#### [MODIFY] [test_println.omni](file:///d:/GitHub/OmniLang/tests/test_println.omni)
- Mengubah isinya menjadi skenario unit test v1.0 yang valid.

---

## Verification Plan

### Automated Tests
- Menjalankan parser terhadap file yang telah dimigrasi untuk memastikan tidak ada error parsing.
- Karena belum ada framework test formal yang terlihat, saya akan mencoba menjalankan build atau manual parsing script jika tersedia.

### Manual Verification
- Memeriksa output AST hasil parsing (jika ada tool CLI yang bisa digunakan).
- Memastikan urutan *Canonical Order* pada file contoh sudah benar sesuai `SPEC_V1.0.md`.

---

## IR/ABI Draft (Target Wasm/Native)
- **IR Level:** `PolicyIR { sections: canonical order, rules: [RuleIR], flat_rules, constraints, impacts }` dengan `RuleIR` sudah ter-normalisasi (IF/THEN, loop body flatten, guard metadata). **Status: Implemented v1.1**.
- **ABI Input:** `evaluate(policy_ir: JSON, context: JSON map<string, number|string>) -> Decision { actions, logs, guard_triggered }`.
- **Determinisme:** Tidak ada akses I/O eksternal; host-call terbatas untuk time/metrics disiapkan via capability flag.
- **Wasm Path (rencana):** Compile runtime evaluator ke Wasm; ekspor fungsi `evaluate` dengan serde-compatible JSON (via wasm-bindgen/wasmtime env WASI minimal).
- **Native Path:** Reuse runtime evaluator; ABI sama dengan Wasm untuk parity.
- **Forward Compatibility:** Tambahkan `capabilities: ["parse", "runtime-eval"]` dan `compiler: "not-available"` pada respons API/UI sebagai deklarasi scope sementara.
