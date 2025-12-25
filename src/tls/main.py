"""Typer CLI application entry point for tls."""

from importlib import metadata
from typing import Optional

import typer
from rich.console import Console

from tls.commands.init import init
from tls.commands.run import run

console = Console()


def get_safe_version(package_name: str, fallback: str = "0.1.0") -> str:
    """
    Safely get the version of a package.

    Args:
        package_name: Name of the package
        fallback: Default version if retrieval fails

    Returns:
        Version string
    """
    try:
        return metadata.version(package_name)
    except metadata.PackageNotFoundError:
        return fallback


def version_callback(value: bool | None) -> None:
    """Print version and exit."""
    if value:
        version = get_safe_version("tls")
        console.print(f"tls version: {version}")
        raise typer.Exit()


app = typer.Typer(
    name="tls",
    help="A Python CLI tool for LLM benchmarking and evaluation.",
    no_args_is_help=True,
)

# Register commands
app.command("init")(init)
app.command("run")(run)


@app.callback()
def main(
    version: Optional[bool] = typer.Option(
        None,
        "--version",
        "-v",
        callback=version_callback,
        is_eager=True,
        help="Show version and exit.",
    ),
) -> None:
    """
    tls - LLM Benchmarking and Evaluation Tool.

    Use subcommands to interact with the application.
    """
    pass


if __name__ == "__main__":
    app()
