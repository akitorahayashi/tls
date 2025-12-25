# ==============================================================================
# justfile for typ-tmpl automation
# ==============================================================================

set dotenv-load

APP_NAME := env("TYP_TMPL_APP_NAME", "typ-tmpl")

# default target
default: help

# Show available recipes
help:
    @echo "Usage: just [recipe]"
    @echo "Available recipes:"
    @just --list | tail -n +2 | awk '{printf "  \033[36m%-20s\033[0m %s\n", $1, substr($0, index($0, $2))}'

# ==============================================================================
# Environment Setup
# ==============================================================================

# Initialize project: install dependencies and create the .env file
setup:
    @echo "🐍 Installing python dependencies with uv..."
    @uv sync
    @echo "Creating environment file..."
    @if [ ! -f .env ] && [ -f .env.example ]; then \
        echo "Creating .env from .env.example..."; \
        cp .env.example .env; \
        echo "✅ Environment file created (.env)"; \
    else \
        echo ".env already exists or .env.example not found. Skipping creation."; \
    fi

# ==============================================================================
# Development Commands
# ==============================================================================

# Run the CLI application
run *args:
    @uv run python -m typ_tmpl {{args}}

# ==============================================================================
# CODE QUALITY
# ==============================================================================

# Automatically format and fix code (Ruff)
fix:
    @echo "🔧 Formatting and fixing code..."
    @uv run ruff format .
    @uv run ruff check . --fix

# Run static checks (Ruff, Mypy)
check: fix
    @echo "🔍 Running static checks..."
    @uv run ruff format --check .
    @uv run ruff check .
    @uv run mypy .

# ==============================================================================
# TESTING
# ==============================================================================

# Run complete test suite
test:
    @just unit-test
    @just intg-test
    @echo "✅ All tests passed!"

# Alias for test (backward compatibility)
local-test:
    @just test

# Run unit tests
unit-test:
    @echo "🚀 Running unit tests..."
    @uv run pytest tests/unit

# Run integration tests
intg-test:
    @echo "🚀 Running integration tests..."
    @uv run pytest tests/intg

# ==============================================================================
# CLEANUP
# ==============================================================================

# Remove __pycache__ and .venv to make project lightweight
clean:
    @echo "🧹 Cleaning up project..."
    @find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
    @rm -rf .venv
    @rm -rf .pytest_cache
    @rm -rf .ruff_cache
    @rm -rf .aider.tags.cache.v4
    @rm -rf .serena/cache
    @rm -rf .uv-cache
    @rm -rf .tmp
    @echo "✅ Cleanup completed"
