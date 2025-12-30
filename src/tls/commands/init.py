"""Init command implementation."""

from pathlib import Path

import typer

from tls.context import AppContext


def init(
    ctx: typer.Context,
    path: Path = typer.Argument(
        Path("."),
        help="Directory to initialize the project in.",
    ),
) -> None:
    """
    Initialize a new tls project.

    Creates the project structure with:
    - telescope.ini configuration file
    - benchmarks/ directory with example files
    - reports/ directory for run outputs
    - .gitignore with appropriate entries
    """
    app_ctx: AppContext = ctx.obj
    console = app_ctx.console
    initializer = app_ctx.initializer

    try:
        report = initializer.execute(path)

        if report.created_paths:
            console.print("[green]✓[/green] Created:")
            for p in report.created_paths:
                console.print(f"  [dim]{p}[/dim]")

        if report.gitignore_updated:
            console.print("[green]✓[/green] Updated .gitignore")

        if report.git_initialized:
            console.print("[green]✓[/green] Initialized git repository")

        console.print("\n[bold green]Project initialized successfully![/bold green]")
        console.print(
            "\nNext steps:\n"
            "  1. Edit [cyan]telescope.ini[/cyan] to configure your models\n"
            "  2. Run [cyan]tls run[/cyan] to execute benchmarks"
        )

    except Exception as e:
        console.print(f"[red]Error:[/red] {e}")
        raise typer.Exit(1)
