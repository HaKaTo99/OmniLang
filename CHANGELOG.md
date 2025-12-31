# OmniLang - Changelog

## [1.0.0] - 2025-12-31
**Stable Release: The Universal Intent Language**

### Added
- **Core Engine Stabilization**: Parser, Lexer, and Evaluator fully optimized for high-assurance policy execution.
- **Pattern Matching**: Implemented `MATCH` expression for complex logic branching.
- **Looping Constructs**: Native support for `FOR` and `WHILE` loops in the Rust backend.
- **IR & Runtime**: Complete IR (JSON) emitter and cross-platform runners (Native & WebAssembly).
- **Standard Library (v1.0)**: 9 core modules including `math`, `crypto`, `tensor`, `web`, and `time`.
- **Observability**: Integrated Prometheus/OpenMetrics exporting and trace-aware structured logging.
- **Ecosystem CI**: Automated GitHub Actions workflow for linting, testing, and multi-platform builds.

### Changed
- **Paradigm Shift**: Transitioned from general-purpose programming to **Universal Intent Language** with canonical section validation (`INTENT:`, `ACTOR:`, `RULE:`, etc.).
- **Backend Overhaul**: Unified AST and refined token-matching for 100% grammar compliance.

---

### [v0.1.0] - 2025-12-10 (Deprecated)
**Initial General Purpose Prototypes**
- Initial lexer and AST structures.
- *Replaced by the v1.0 architecture.*
