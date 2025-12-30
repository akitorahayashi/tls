"""Integration tests for CLI commands."""

import tempfile
from pathlib import Path

from typer.testing import CliRunner

from tls.main import app


class TestCLIIntegration:
    """Integration tests for CLI command interactions."""

    def test_version_flag_shows_version(self, cli_runner: CliRunner) -> None:
        """Test that --version flag shows version information."""
        result = cli_runner.invoke(app, ["--version"])
        assert result.exit_code == 0
        assert "tls version:" in result.output

    def test_help_flag_shows_help(self, cli_runner: CliRunner) -> None:
        """Test that --help flag shows help information."""
        result = cli_runner.invoke(app, ["--help"])
        assert result.exit_code == 0
        assert "tls" in result.output

    def test_init_help_shows_usage(self, cli_runner: CliRunner) -> None:
        """Test that init --help shows usage information."""
        result = cli_runner.invoke(app, ["init", "--help"])
        assert result.exit_code == 0
        assert "Initialize" in result.output

    def test_run_help_shows_usage(self, cli_runner: CliRunner) -> None:
        """Test that run --help shows usage information."""
        result = cli_runner.invoke(app, ["run", "--help"])
        assert result.exit_code == 0
        assert "benchmark" in result.output.lower()


class TestInitCommand:
    """Integration tests for the init command."""

    def test_init_creates_project_structure(self, cli_runner: CliRunner) -> None:
        """Test that init creates proper project structure."""
        with tempfile.TemporaryDirectory() as tmpdir:
            project_dir = Path(tmpdir) / "new-project"
            result = cli_runner.invoke(app, ["init", str(project_dir)])

            assert result.exit_code == 0
            assert (project_dir / "telescope.ini").exists()
            assert (project_dir / "benchmarks").is_dir()
            assert (project_dir / "reports").is_dir()

    def test_init_in_current_directory(self, cli_runner: CliRunner) -> None:
        """Test that init works with default current directory."""
        with tempfile.TemporaryDirectory() as tmpdir:
            import os

            original_dir = os.getcwd()
            try:
                os.chdir(tmpdir)
                result = cli_runner.invoke(app, ["init"])

                assert result.exit_code == 0
                assert (Path(tmpdir) / "telescope.ini").exists()
            finally:
                os.chdir(original_dir)


class TestRunCommand:
    """Integration tests for the run command."""

    def test_run_without_config_shows_error(self, cli_runner: CliRunner) -> None:
        """Test that run without telescope.ini shows helpful error."""
        with tempfile.TemporaryDirectory() as tmpdir:
            import os

            original_dir = os.getcwd()
            try:
                os.chdir(tmpdir)
                result = cli_runner.invoke(app, ["run"])

                assert result.exit_code == 1
                assert (
                    "telescope.ini" in result.output.lower()
                    or "error" in result.output.lower()
                )
            finally:
                os.chdir(original_dir)
