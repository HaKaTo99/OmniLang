# Changelog OmniLang (omc)

Semua perubahan signifikan pada proyek ini akan didokumentasikan di file ini.

## [2.1.0-Distributed-Fabric] - 2026-03-01
### Added
- **Native AI Inference (`@oracle`)**: Fungsionalitas _proxy engine_ ONNX untuk membedah komputasi Model Jaringan Saraf Tiruan (*Neural Networks*) berbasis tensor. Mendukung sistem deklarasi tanpa bingkai pustaka luar (FFI-less).
- **Secure Distributed Mesh (`@mesh`)**: Modul pilar arsitektur TCP Remote Procedure Call tersinkronisasi murni di dalam runtime, memfasilitasi komunikasi Client-Worker-Actuator yang independen.
- **X-Capability Security**: Transmisi pemanggilan jarak jauh disisipi oleh "token kapabilitas" bawaan dari sisi _Client_ dan diverifikasi teguh di sisi Server (_Worker_).
- **Orchestration Tutorial**: Penambahan panduan resmi _Distributed Mesh_ dan `onnx_10min` di dalam `docs/tutorials`.

### Changed
- **Argument CLI**: Ekspansi `omnilang serve --port <PORT> --token <TOKEN>` untuk menjadikan OmniLang sebagai daemon pekerja.
- **Program Evaluator Engine**: Evaluator kini memiliki mode `--worker` dan kemampuan translasi serialisasi JSON AST yang melintasi transport TCP antar port.

## [1.2.1] - 2026-02-19
### Added
- **Audit & Synchronization**: Audit menyeluruh terhadap seluruh komponen proyek (Core, omc, Web, Docs).
- **Stability Phase**: Penambahan Fase 1.3 pada Roadmap untuk perbaikan infrastruktur.
- **Next Steps Guide**: Pembuatan panduan perbaikan stabilitas di `docs/architecture/NEXT_STEPS.md`.

### Changed
- **Versioning**: Sinkronisasi versi proyek menjadi 1.2.0 di seluruh tumpukan teknologi.
- **Documentation Refactor**: Pengarsipan rencana implementasi lama dan harmonisasi narasi strategi.

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
