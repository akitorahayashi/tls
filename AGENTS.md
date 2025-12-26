# tls (Telescope) Agent Notes

## Overview
- Python CLI tool for LLM benchmarking and evaluation.
- Enables systematic testing of language models against structured benchmarks with progress tracking and detailed reporting.
- Fully async architecture using `httpx` and `aiofiles` for I/O operations.

## Design Philosophy
- **Single Source of Truth**: Pydantic models define all benchmark-related data structures.
- **Type Safety**: Uses Pydantic model serialization for schema correctness.
- **Dependency Injection**: Central DI container manages services (`LlmClient`, `Reporter`, `Executor`).
- **Async First**: Core I/O operations implemented asynchronously using `asyncio`.

## Key Commands
- `tls init [PATH]` - Initialize a new benchmark project with configuration and example files.
- `tls run [OPTIONS]` - Execute benchmark evaluations against configured LLM models.

## Key Files
- `src/tls/main.py`: Typer app instantiation; command registration.
- `src/tls/commands/init.py`: Project initialization logic.
- `src/tls/commands/run.py`: Benchmark execution command.
- `src/tls/models/`: Pydantic models for benchmarks, reports, and configuration.
- `src/tls/services/executor.py`: Core benchmark execution engine.
- `src/tls/services/llm_client.py`: HTTP client for OpenAI-compatible APIs.
- `src/tls/services/reporter.py`: Report writing (filesystem, in-memory).
- `src/tls/config/settings.py`: Configuration loading from `telescope.ini`.
- `src/tls/config/templates.py`: Default templates for project initialization.

## Configuration
- Uses `telescope.ini` (INI format) for project configuration.
- Environment variables: `TLS_APP_NAME`, `TLS_USE_MOCK_LLM`.

## Tooling
- `justfile`: run/lint/test tasks. Use `just test` as the unified entrypoint.
- `uv`: Dependency management. Run `uv sync` to install dependencies.
