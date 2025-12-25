"""Unit tests for the greeting services."""

from dev.mocks.services.mock_greeting_service import MockGreetingService
from typ_tmpl.config.settings import AppSettings
from typ_tmpl.core.container import create_container
from typ_tmpl.services.greeting_service import GreetingService


class TestGreetingService:
    """Unit tests for greeting services."""

    def test_greeting_service_generate_greeting(self):
        service = GreetingService()
        result = service.generate_greeting("World")
        assert result == "Hello, World"

    def test_mock_greeting_service_generate_greeting(self):
        service = MockGreetingService()
        result = service.generate_greeting("Developers")
        assert result == "[mock] Hello, Developers"


class TestContainer:
    """Unit tests for the DI container."""

    def test_create_container_defaults_to_real_service(self, monkeypatch):
        monkeypatch.delenv("TYP_TMPL_USE_MOCK_GREETING", raising=False)

        settings = AppSettings()
        container = create_container(settings)

        assert isinstance(container.greeting_service, GreetingService)

    def test_create_container_uses_mock_when_enabled(self, monkeypatch):
        monkeypatch.setenv("TYP_TMPL_USE_MOCK_GREETING", "true")

        settings = AppSettings()
        container = create_container(settings)

        assert isinstance(container.greeting_service, MockGreetingService)
