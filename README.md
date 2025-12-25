# tls

`tls` (Telescope) is a Python CLI tool for LLM benchmarking and evaluation. It enables systematic testing of language models against structured benchmarks with progress tracking and detailed reporting.

## 🚀 Installation

### Install with pipx (Recommended)

Install directly from GitHub using [pipx](https://pipx.pypa.io/):

```shell
pipx install git+https://github.com/akitorahayashi/tls.git
```

After installation, the `tls` command is available globally:

```shell
tls --version
tls --help
```

### Development Setup

For development, clone the repository and use [uv](https://github.com/astral-sh/uv):

```shell
git clone https://github.com/akitorahayashi/tls.git
cd tls
just setup
```

This installs dependencies with `uv` and creates a local `.env` file if one does not exist.

## 🔧 Usage

### Initialize a New Project

```shell
tls init [PATH]
```

Creates the project structure:
- `telescope.ini` - Configuration file
- `benchmarks/` - Directory for benchmark JSON files
- `reports/` - Directory for run outputs

### Run Benchmarks

```shell
tls run [OPTIONS]
```

Options:
- `--blocks, -b PATH` - Directory containing benchmark files
- `--file, -f PATH` - Specific benchmark file to run
- `--id, -i TEXT` - Specific test case ID to run
- `--model, -m TEXT` - Model(s) to use (can be repeated)
- `--timeout, -t INT` - Request timeout in seconds

### Configuration

Edit `telescope.ini` to configure your project:

```ini
[project]
name = my-project
description = My evaluation project
blocks_dir = ./benchmarks

[target]
models = qwen3-vl:8b-instruct-q4_K_M
endpoint = http://127.0.0.1:11434
timeout = 300
# api_key = your-api-key-here
```

### Run during Development

```shell
just run --help
just run init
just run run --model gpt-4
```

Or directly via Python:

```shell
uv run python -m tls --help
uv run python -m tls init
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
src/
└── tls/
    ├── __init__.py
    ├── __main__.py      # python -m tls entry point
    ├── main.py          # Typer app factory and command registration
    ├── commands/
    │   ├── init.py      # Project initialization command
    │   └── run.py       # Benchmark execution command
    ├── config/
    │   ├── settings.py  # Pydantic settings and config loader
    │   └── templates.py # Default templates for initialization
    ├── core/
    │   ├── container.py # DI container and context
    │   └── exceptions.py # Custom exception classes
    ├── models/          # Pydantic models for benchmarks, reports, config
    └── services/        # Business logic services
tests/
│   ├── unit/                # Pure unit tests (service layer)
│   └── intg/                # Integration tests (CLI with CliRunner)
├── justfile
└── pyproject.toml
```

## 🔧 Environment Variables

Environment variables are loaded from `.env` (managed by `just setup`):

- `TLS_APP_NAME` – application display name (default `tls`).
- `TLS_USE_MOCK_LLM` – when `true`, injects the mock LLM client for testing.

## 📊 Benchmark Format

Benchmark files are JSON documents with the following structure:

```json
{
  "metadata": {
    "id": "my-benchmark",
    "description": "Description of the benchmark",
    "active": true
  },
  "prompts": {
    "system": "You are a helpful assistant."
  },
  "dataset": [
    {
      "id": "case-001",
      "input": "What is 2+2?",
      "expected": "4"
    }
  ]
}
```
