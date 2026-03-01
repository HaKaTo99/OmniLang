# Next Steps: Stability & Infrastructure (Feb 2026)

Berdasarkan audit stabilitas dan kekuatan Mesh AI terkini, berikut adalah langkah-langkah prioritas yang harus diambil untuk mengejar kematangan v2.2.0 (Platform Saturation: WASM/JVM).

## Proposed Changes

### 1. Restrukturisasi Folder `src/` [CRITICAL]
- **Tujuan**: Memisahkan kode Rust Core Engine dari kode Web Studio.
- **Tindakan**:
  - Buat folder `web/` di root.
  - Pindahkan `src/app`, `src/components`, `src/hooks`, dan `src/lib/placeholder-images.ts` ke `web/`.
  - Sisakan `src/*.rs`, `src/stdlib/`, dan `src/ai/` (jika digunakan oleh Rust) di folder `src/`.

### 2. Perbaikan Frontend Web Studio [HIGH]
- **Tujuan**: Memperbaiki syntax error yang ditemukan di audit.
- **Tindakan**:
  - Perbaiki `src/components/header.tsx` (yang akan pindah ke `web/components/header.tsx`) pada baris 36-40.
  - Pastikan semua dependensi UI (`Button`, `CheckCircle2`) diimpor dengan benar di `CodeEditorPanel`.

### 3. Debugging Engine & Examples [HIGH]
- **Tujuan**: Memastikan semua contoh kode lulus uji validasi.
- **Tindakan**:
  - Analisis kegagalan pada `3d_game_demo.omni` dan `lambda_hof_v2.omni`.
  - Perbaiki `evaluator.rs` atau `runtime.rs` jika terdapat ketidakkonsistenan penanganan HOF atau unit suffix.

## Verification Plan
1. Jalankan `cargo test` untuk memverifikasi konsistensi Core Engine.
2. Jalankan `npm run dev` (setelah update path di `package.json`/`tsconfig.json`) untuk memverifikasi Web Studio.
3. Jalankan `test_all.ps1` dan pastikan `test_results.csv` menunjukkan 100% SUCCESS.
