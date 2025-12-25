"""Integration tests for CLI command interactions."""

from typer.testing import CliRunner

from typ_tmpl.main import app


class TestCLIIntegration:
    """Integration tests for CLI command interactions."""

    def test_version_flag_shows_version(self, cli_runner: CliRunner):
        """Test that --version flag shows version information."""
        result = cli_runner.invoke(app, ["--version"])

        assert result.exit_code == 0
        assert "typ-tmpl version:" in result.output

    def test_help_flag_shows_help(self, cli_runner: CliRunner):
        """Test that --help flag shows help information."""
        result = cli_runner.invoke(app, ["--help"])

        assert result.exit_code == 0
        assert "typ-tmpl" in result.output

    def test_greet_hello_with_real_service(self, cli_runner: CliRunner):
        """Test that the greet command works with the real service."""
        # Use env parameter to override environment variable for this invocation
        result = cli_runner.invoke(
            app,
            ["greet", "hello", "Alice"],
            env={"TYP_TMPL_USE_MOCK_GREETING": "false"},
        )

        assert result.exit_code == 0
        assert "Hello, Alice" in result.output

    def test_greet_hello_with_mock_service(self, cli_runner: CliRunner):
        """Test that the greet command works with the mock service."""
        result = cli_runner.invoke(
            app,
            ["greet", "hello", "World"],
            env={"TYP_TMPL_USE_MOCK_GREETING": "true"},
        )

        assert result.exit_code == 0
        assert "[mock] Hello, World" in result.output

    def test_greet_hello_missing_name_shows_error(self, cli_runner: CliRunner):
        """Test that missing name argument shows an error."""
        result = cli_runner.invoke(app, ["greet", "hello"])

        assert result.exit_code != 0

    def test_no_args_shows_help(self, cli_runner: CliRunner):
        """Test that running without arguments shows help (exit code 0 or 2)."""
        result = cli_runner.invoke(app, [])

        # no_args_is_help=True causes exit code 0, but if help is shown as error it's 2
        assert "Usage:" in result.output or "typ-tmpl" in result.output
