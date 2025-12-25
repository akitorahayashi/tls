"""Greeting command implementation."""

import typer

from typ_tmpl.core.container import AppContext

greet_app = typer.Typer(help="Greeting commands.")


@greet_app.command("hello")
def hello(
    ctx: typer.Context,
    name: str = typer.Argument(..., help="Name of the person to greet."),
) -> None:
    """
    Greet someone by name.

    Example:
        typ-tmpl greet hello World
    """
    app_ctx: AppContext = ctx.obj
    message = app_ctx.greeting_service.generate_greeting(name)
    typer.echo(message)
