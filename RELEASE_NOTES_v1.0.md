# Release Notes — OmniLang v1.0 (2025-12-31)

This release marks the stabilization of OmniLang core for intent validation and policy execution.

## Highlights

- Core parser/lexer and evaluator stabilized — full support for rules, loops, and match expressions.
- IR emitter and runtime runner for native and wasm completed and smoke-tested.
- Stdlib: math, string, time, json, crypto (basic), tensor ops (dot/matmul).
- Observability: basic OpenMetrics exporter and trace-aware logs.
- OmniRoutine scheduler for parallel action execution.
- Comprehensive test suite: all unit and integration tests passing locally.
- CI: GitHub Actions workflow added to run `cargo test`, `clippy`, `fmt`, and builds.

## Known issues

- LSP / VSCode extension needs polishing before public publish.
- Additional stdlib features (secure RNG, more tensor ops) planned.
- Ecosystem integration (ROS2, K8s admission) targeted for post-release.

## Upgrade notes

- Consumers using `examples/` should run `cargo test --all` to validate their environment.

## Contributors

Thanks to everyone who contributed to the v1.0 stabilization sprint.
