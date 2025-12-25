# typ-tmpl

`typ-tmpl` is a minimal, database-independent Python CLI template using Typer. It provides a clean scaffold with dependency injection via context objects, protocols for service interfaces, and a factory pattern for services. This enables high extensibility, maintainability, and testability. Includes environment-aware configuration and a lightweight test suite so you can start new CLI tools quickly without dragging in domain-specific code.

## 🚀 Installation

### Install with pipx (Recommended)

Install directly from GitHub using [pipx](https://pipx.pypa.io/):

```shell
pipx install git+https://github.com/akitorahayashi/typ-tmpl.git
```

After installation, the `typ-tmpl` command is available globally:

```shell
typ-tmpl --version
typ-tmpl --help
typ-tmpl greet hello World
```

### Development Setup

For development, clone the repository and use [uv](https://github.com/astral-sh/uv):

```shell
git clone https://github.com/akitorahayashi/typ-tmpl.git
cd typ-tmpl
just setup
```

This installs dependencies with `uv` and creates a local `.env` file if one does not exist.

### Run during Development

```shell
just run --help
just run greet hello World
just run --version
```

Or directly via Python:

```shell
uv run python -m typ_tmpl --help
uv run python -m typ_tmpl greet hello World
```

### Run Tests and Linters

```shell
just test       # run all tests (unit + intg)
just unit-test  # run unit tests only
just intg-test  # run integration tests only
just check      # ruff format --check, ruff check, and mypy
just fix        # auto-format with ruff format and ruff --fix
```

## 🧱 Project Structure

```
├── dev/
│   └── mocks/
│       └── services/
│           └── mock_greeting_service.py  # Toggleable mock implementation
├── src/
│   └── typ_tmpl/
│       ├── __init__.py
│       ├── __main__.py      # python -m typ_tmpl entry point
│       ├── main.py          # Typer app factory and command registration
│       ├── commands/
│       │   └── greet.py     # Greeting command implementation
│       ├── config/
│       │   └── settings.py  # Pydantic settings
│       ├── core/
│       │   └── container.py # DI container and context
│       ├── protocols/       # Protocol definitions for service interfaces
│       └── services/        # Concrete service implementations
├── tests/
│   ├── unit/                # Pure unit tests (service layer)
│   └── intg/                # Integration tests (CLI with CliRunner)
├── justfile
└── pyproject.toml
```

## 🔧 Configuration

Environment variables are loaded from `.env` (managed by `just setup`):

- `TYP_TMPL_APP_NAME` – application display name (default `typ-tmpl`).
- `TYP_TMPL_USE_MOCK_GREETING` – when `true`, injects the development mock greeting service.

## ✅ Commands

The template ships with greeting commands:

```shell
typ-tmpl --version           # Show version
typ-tmpl --help              # Show help
typ-tmpl greet hello <name>  # Greet someone by name
```

Use this as a foundation for adding your own commands, services, and business logic.

