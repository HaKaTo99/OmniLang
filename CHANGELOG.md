# Changelog OmniLang (omc)

Semua perubahan signifikan pada proyek ini akan didokumentasikan di file ini.

## [1.0.0-rc1] - 2024-05-24
### Added
- **Phase 1-2**: Lexer & Parser (Hand-written Recursive Descent).
- **Phase 3**: Semantic Analyzer dengan Symbol Table and scope management.
- **Phase 4**: OmniIR (Intermediate Representation) dengan dukungan control flow via labels.
- **Phase 5**: Backend Rust Generator dengan State Machine yang sangat stabil (military-grade).
- **Phase 6**: CLI Integration yang menghasilkan file `output.rs` yang dapat dikompilasi dengan `rustc`.
- **Phase 7**: Standard Library dasar: fungsi `print` built-in.
- **Phase 8**: Optimasi IR: Constant Folding (evaluasi aritmatika saat kompilasi).

### Changed
- Refactor `RustGenerator` untuk mendukung variable hoisting (menghindari error scope pada Rust).
- Perbaikan pada pemetaan tipe data Boolean ke `i64` di IR untuk konsistensi register.

### Fixed
- Perbaikan bug pada penanganan `return` dan `break` di dalam loop state machine.
- Perbaikan error `Undefined variable` pada fungsi built-in.
