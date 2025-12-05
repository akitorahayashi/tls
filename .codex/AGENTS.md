# Telescope (tls) Development Overview

## Project Summary
Telescope (`tls`) is a local-first CLI tool for running LLM prompts against local endpoints like Ollama or MLX. It provides a clean architecture for managing benchmark blocks and execution logs, enabling quick prompt testing and iteration during development.

## Tech Stack
- **Language**: Rust
- **CLI Parsing**: `clap`
- **HTTP Client**: `reqwest`
- **Async Runtime**: `tokio`
- **Development Dependencies**:
  - `assert_cmd`
  - `predicates`
  - `tempfile`
  - `wiremock`

## Coding Standards
- **Formatter**: `rustfmt` is used for code formatting. Key rules include a maximum line width of 100 characters, crate-level import granularity, and grouping imports by standard, external, and crate modules.
- **Linter**: `clippy` is used for linting, with a strict policy of treating all warnings as errors (`-D warnings`).

## Naming Conventions
- **Structs and Enums**: `PascalCase` (e.g., `Cli`, `Commands`)
- **Functions and Variables**: `snake_case` (e.g., `run_blocks`, `test_context`)
- **Modules**: `snake_case` (e.g., `gateway.rs`, `storage.rs`)

## Key Commands
- **Build (Debug)**: `cargo build`
- **Build (Release)**: `cargo build --release`
- **Format Check**: `cargo fmt --check`
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Test**: `RUST_TEST_THREADS=1 cargo test --all-targets --all-features`

## Testing Strategy
- **Unit Tests**: Located within the `src/` directory alongside the code they test, covering gateway client and storage logic.
- **Core Logic Tests**: Found in `src/core/`, utilizing mock clients to ensure business logic is tested in isolation.
- **Integration Tests**: Housed in the `tests/` directory, these tests cover CLI workflows from an external perspective:
  - `tests/init.rs` - workspace initialization tests
  - `tests/run.rs` - run command tests with mock LLM endpoints
  - `tests/common/mod.rs` - shared test utilities
- **CI**: GitHub Actions automatically runs build, linting, and test workflows.

## Architectural Highlights
- **Two-tier structure**: `src/main.rs` handles CLI parsing, `src/commands.rs` wires dependencies and user messaging, and `src/core/runner.rs` contains the run logic.
- **Gateway abstraction**: `src/gateway.rs` defines a `GenAiClient` trait for LLM communication, defaulting to local Ollama endpoint.
- **Storage Layout**: Workspaces use `.telescope/runs/` for execution logs, with `benchmarks/` and `metrics/` for test blocks.

## Environment Variables
- `OPENAI_API_COMPATIBLE_LLM_ENDPOINT`: Base URL for the LLM API (default: `http://127.0.0.1:11434`)
- `OPENAI_API_KEY`: API key (optional for local LLMs, defaults to `"dummy"`)
