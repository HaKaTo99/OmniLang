# OmniLang - Changelog

## Versi Terkini

### [v1.0.0] - 2025-12-25
**Migrasi Besar: Universal Intent Language**
- 🚀 **Shift Paradigma**: Transformasi dari bahasa pemrograman umum ke "Universal Intent Language" untuk definisi kebijakan dan niat.
- ✨ **Sintaks Baru**: Implementasi keyword seksi (`INTENT:`, `ACTOR:`, `RULE:`, dll) dengan validasi *Canonical Order*.
- 🛠️ **Backend Sync**: Pembaruan AST, Lexer, dan Parser di Rust untuk mendukung sintaks v1.0.
- 💻 **Frontend Sync**: Pembaruan `OmniLang Studio` dengan validasi real-time untuk skema kebijakan v1.0.
- 📚 **Dokumentasi**: Peluncuran `SPEC_V1.0.md` sebagai standar baru.

### [v0.1.0] - 2025-12-10 (Deprecated)
**Fitur Awal (General Purpose)**
- ✅ Lexer dengan dukungan token lengkap (mod, struct, func, let, dll)
- ✅ Parser dengan AST untuk modul, struct, function
- ✅ Interpreter dasar
- ⚠️ *Versi ini sudah digantikan oleh arsitektur v1.0.*
