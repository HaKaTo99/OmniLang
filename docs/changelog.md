# Changelog OmniLang (omc)

Semua perubahan signifikan pada proyek ini akan didokumentasikan di file ini.

## [1.2.0-Harmonious] - 2026-02-16
### Added
- **Harmonious Singularity**: Sinkronisasi penuh antara Core Engine (Deklaratif) dan omc Compiler (Imperatif).
- **Universal Access (7+ Interfaces)**: CLI, TUI, GUI, VUI, NUI, CUI, dan HUI (Serial/UART).
- **Futuristic Context**: Placeholder fungsional untuk BCI (`--bci`) dan PUI (`--pui`).
- **Platform Saturation Vision**: Roadmap untuk Legacy Mobile (Symbian/BB), Unix, dan Database.
- **TUI Chat Integration**: Tab baru untuk interaksi CUI langsung di dalam IDE terminal.

### Improved
- **Stability**: Penanganan panic menggunakan `catch_unwind` pada proses kompilasi TUI.
- **Aesthetics**: Banner terminal "Singularity" yang lebih premium dan status bar informatif.

## [1.1.0] - 2026-02-12
### Added
- **Cyber IDE (TUI)**: Dashboard interaktif berbasis Ratatui dengan panel Source, IR, dan Rust.
- **Syntax Highlighting**: Penomoran baris dan penyorotan logika di editor terminal.
- **Multi-File Navigation**: Sidebar explorer untuk berpindah antar file `.omni`.
- **Command Palette**: Dukungan command visual (`exec`, `lint`, `load`, dll).


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
