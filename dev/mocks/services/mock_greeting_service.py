"""Mock greeting service for development-time toggles."""

from typ_tmpl.protocols.greeting_service_protocol import GreetingServiceProtocol


class MockGreetingService(GreetingServiceProtocol):
    """Return a clearly identifiable greeting for tests and demos."""

    def generate_greeting(self, name: str) -> str:
        return f"[mock] Hello, {name}"


_: GreetingServiceProtocol = MockGreetingService()
