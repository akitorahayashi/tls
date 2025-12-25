"""Typer CLI application entry point for typ-tmpl."""

from importlib import metadata
from typing import Optional

import typer
from rich.console import Console

from typ_tmpl.commands.greet import greet_app
from typ_tmpl.core.container import create_container

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
        version = get_safe_version("typ-tmpl")
        console.print(f"typ-tmpl version: {version}")
        raise typer.Exit()


app = typer.Typer(
    name="typ-tmpl",
    help="A minimal, database-independent Python CLI template using Typer.",
    no_args_is_help=True,
)

# Register sub-command groups
app.add_typer(greet_app, name="greet")


@app.callback()
def main(
    ctx: typer.Context,
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
    typ-tmpl - A minimal Python CLI template.

    Use subcommands to interact with the application.
    """
    # Initialize the application context
    ctx.obj = create_container()


if __name__ == "__main__":
    app()
