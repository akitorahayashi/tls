"""Dependency injection container for the Typer CLI application."""

from dataclasses import dataclass

from typ_tmpl.config.settings import AppSettings
from typ_tmpl.protocols.greeting_service_protocol import GreetingServiceProtocol


@dataclass
class AppContext:
    """Application context holding settings and service instances."""

    settings: AppSettings
    greeting_service: GreetingServiceProtocol


def get_greeting_service(settings: AppSettings) -> GreetingServiceProtocol:
    """
    Get the appropriate greeting service based on settings.

    This function handles the conditional import of mock services to avoid
    importing development code in production scenarios.

    Args:
        settings: Application settings containing the mock toggle.

    Returns:
        Either a mock or real greeting service implementation.
    """
    if settings.use_mock_greeting:
        # Import mock only when needed to avoid dev code in production builds
        from dev.mocks.services.mock_greeting_service import MockGreetingService

        return MockGreetingService()
    else:
        from typ_tmpl.services.greeting_service import GreetingService

        return GreetingService()


def create_container(
    settings: AppSettings | None = None,
    greeting_service: GreetingServiceProtocol | None = None,
) -> AppContext:
    """
    Create and return the application context with all dependencies wired.

    Args:
        settings: Optional pre-configured settings. If None, loads from environment.
        greeting_service: Optional service override for testing. If None, uses
                         the appropriate service based on settings.

    Returns:
        AppContext with settings and services initialized.
    """
    if settings is None:
        settings = AppSettings()

    if greeting_service is None:
        greeting_service = get_greeting_service(settings)

    return AppContext(settings=settings, greeting_service=greeting_service)
