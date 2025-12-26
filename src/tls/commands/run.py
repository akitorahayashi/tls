"""Run command implementation."""

import asyncio
from pathlib import Path

import typer
from rich.console import Console

from tls.config.settings import load_config
from tls.core.exceptions import ConfigError, TlsError
from tls.services.executor import Executor
from tls.services.llm_client import LlmClient
from tls.services.reporter import FileSystemReporter

console = Console()


def run(
    blocks_dir: Path = typer.Option(
        None,
        "--blocks",
        "-b",
        help="Directory containing benchmark files. Defaults to config value.",
    ),
    file: Path = typer.Option(
        None,
        "--file",
        "-f",
        help="Specific benchmark file to run.",
    ),
    case_id: str = typer.Option(
        None,
        "--id",
        "-i",
        help="Specific test case ID to run.",
    ),
    model: list[str] = typer.Option(
        None,
        "--model",
        "-m",
        help="Model(s) to use. Can be specified multiple times. Defaults to config values.",
    ),
    timeout: int = typer.Option(
        None,
        "--timeout",
        "-t",
        help="Request timeout in seconds. Defaults to config value.",
    ),
) -> None:
    """
    Run benchmark evaluations.

    Executes test cases against configured LLM models and writes
    results to the reports directory.
    """
    try:
        # Load configuration
        project_root = Path.cwd()
        config = load_config(project_root)

        # Override with command-line options
        effective_blocks_dir = blocks_dir or project_root / config.project.blocks_dir
        effective_models = list(model) if model else config.target.models
        effective_timeout = timeout or config.target.timeout

        if not effective_models:
            raise ConfigError(
                "No models specified. Configure in telescope.ini or use --model."
            )

        # Create services
        client = LlmClient(
            base_url=config.target.endpoint,
            api_key=config.target.api_key,
            timeout=effective_timeout,
        )

        reports_dir = project_root / "reports"
        reporter = FileSystemReporter(reports_dir=reports_dir)

        executor = Executor(
            client=client,
            reporter=reporter,
            console=console,
        )

        # Run the benchmarks
        summary = asyncio.run(
            executor.execute(
                blocks_dir=effective_blocks_dir,
                models=effective_models,
                target_file=file,
                target_id=case_id,
            )
        )

        # Print summary
        console.print()
        console.print("[bold]Run Summary[/bold]")
        console.print(f"  Duration: {summary.duration_seconds:.2f}s")
        console.print(f"  Total cases: {summary.total_cases}")
        console.print(f"  [green]Successful: {summary.successful_cases}[/green]")
        if summary.failed_cases > 0:
            console.print(f"  [red]Failed: {summary.failed_cases}[/red]")

        for model_summary in summary.models:
            console.print(f"\n  Model: [cyan]{model_summary.model}[/cyan]")
            if model_summary.run_dir:
                console.print(f"    Report: [dim]{model_summary.run_dir}[/dim]")

    except TlsError as e:
        console.print(f"[red]Error:[/red] {e}")
        raise typer.Exit(1)
    except Exception as e:
        console.print(f"[red]Unexpected error:[/red] {e}")
        raise typer.Exit(1)
