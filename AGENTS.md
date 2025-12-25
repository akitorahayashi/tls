# typ-tmpl Agent Notes

## Overview
- Minimal Typer CLI template intended as a clean starting point for new command-line tools.
- Ships only the essentials: dependency injection using context objects, protocols for interfaces, factory pattern for services, greeting commands, and test/CI wiring.

## Design Philosophy
- Stay database-agnostic; add persistence only when the target project needs it.
- Use Typer-native `ctx.obj` pattern with protocols for service interfaces and factory pattern for implementations to maximize extensibility, maintainability, and testability.
- Keep settings and dependencies explicit via `AppSettings` and `AppContext` container.
- Maintain parity between local and CI flows with a single source of truth (`just`, `uv`, `.env`).

## First Steps When Creating a Real CLI
1. Clone or copy the template and run `just setup` to install dependencies.
2. Rename the Python package from `typ_tmpl` if you need a project-specific namespace.
3. Add new commands under `src/typ_tmpl/commands/` and register them in `main.py`.
4. Update `.env.example` and documentation to reflect new environment variables or external services.

## Key Files
- `src/typ_tmpl/core/container.py`: central place to wire settings and service providers.
- `src/typ_tmpl/main.py`: Typer app instantiation; attach new command groups here.
- `src/typ_tmpl/commands/`: command implementations (subcommand modules).
- `src/typ_tmpl/protocols/`: protocol definitions for service interfaces.
- `src/typ_tmpl/services/`: concrete service implementations.
- `tests/`: unit/intg layout kept light so additional checks can drop in without restructuring.

## Tooling Snapshot
- `justfile`: run/lint/test tasks (`unit-test`, `intg-test`) used locally and in CI. Prefer `just test` as the unified entrypoint.
- `uv.lock` + `pyproject.toml`: reproducible dependency graph; regenerate with `uv pip compile` when deps change.
