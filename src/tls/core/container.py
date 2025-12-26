"""Dependency injection container for the tls CLI application."""

from dataclasses import dataclass
from pathlib import Path

from rich.console import Console

from tls.config.settings import AppSettings, load_config
from tls.core.exceptions import TlsError
from tls.models.project_config import Config
from tls.services.executor import Executor
from tls.services.initializer import Initializer
from tls.services.llm_client import GenAiClient, LlmClient, MockLlmClient
from tls.services.reporter import FileSystemReporter, ReportWriter


@dataclass
class AppContext:
    """Application context holding settings and service instances."""

    settings: AppSettings
    config: Config | None
    llm_client: GenAiClient
    reporter: ReportWriter
    executor: Executor
    initializer: Initializer
    console: Console


def get_llm_client(settings: AppSettings, config: Config | None) -> GenAiClient:
    """
    Get the appropriate LLM client based on settings.

    Args:
        settings: Application settings containing the mock toggle.
        config: Optional project configuration with endpoint details.

    Returns:
        Either a mock or real LLM client implementation.
    """
    if settings.use_mock_llm:
        return MockLlmClient()

    if config is None:
        # Return mock if no config is available
        return MockLlmClient()

    return LlmClient(
        base_url=config.target.endpoint,
        api_key=config.target.api_key,
        timeout=config.target.timeout,
    )


def create_container(
    settings: AppSettings | None = None,
    project_root: Path | None = None,
    llm_client: GenAiClient | None = None,
    reporter: ReportWriter | None = None,
) -> AppContext:
    """
    Create and return the application context with all dependencies wired.

    Args:
        settings: Optional pre-configured settings. If None, loads from environment.
        project_root: Optional project root directory. If None, uses current directory.
        llm_client: Optional LLM client override for testing.
        reporter: Optional reporter override for testing.

    Returns:
        AppContext with settings and services initialized.
    """
    if settings is None:
        settings = AppSettings()

    console = Console()

    # Try to load config, but don't fail if not present
    config: Config | None = None
    if project_root is None:
        project_root = Path.cwd()

    try:
        config = load_config(project_root)
    except TlsError:
        # Config not available or invalid, will use defaults or mocks
        pass

    if llm_client is None:
        llm_client = get_llm_client(settings, config)

    if reporter is None:
        reports_dir = project_root / "reports"
        reporter = FileSystemReporter(reports_dir=reports_dir)

    executor = Executor(
        client=llm_client,
        reporter=reporter,
        console=console,
    )

    initializer = Initializer(console=console)

    return AppContext(
        settings=settings,
        config=config,
        llm_client=llm_client,
        reporter=reporter,
        executor=executor,
        initializer=initializer,
        console=console,
    )
