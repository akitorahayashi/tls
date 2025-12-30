"""Shared pytest fixtures for the tls project."""

from typing import Any, Generator

import pytest
from typer.testing import CliRunner

from tls.main import app


@pytest.fixture(scope="session", autouse=True)
def setup_test_environment() -> Generator[None, None, None]:
    """Setup test environment with dotenv loading."""
    try:
        import dotenv

        dotenv.load_dotenv()
    except ImportError:
        pass
    yield


@pytest.fixture()
def cli_runner() -> CliRunner:
    """Provide a CLI runner for testing Typer commands."""
    return CliRunner()


@pytest.fixture()
def typer_app() -> Any:
    """Return the Typer application under test."""
    return app
