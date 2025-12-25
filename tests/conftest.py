"""Shared pytest fixtures for the typ-tmpl project template."""

import pytest
from typer.testing import CliRunner

from typ_tmpl.main import app


@pytest.fixture(scope="session", autouse=True)
def setup_test_environment():
    """Setup test environment with dotenv loading."""
    try:
        import dotenv

        dotenv.load_dotenv()
    except ImportError:
        pass


@pytest.fixture()
def cli_runner() -> CliRunner:
    """Provide a CLI runner for testing Typer commands."""
    return CliRunner()


@pytest.fixture()
def typer_app():
    """Return the Typer application under test."""
    return app
