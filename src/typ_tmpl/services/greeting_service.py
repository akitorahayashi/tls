"""Concrete greeting service used by default."""

from typ_tmpl.protocols.greeting_service_protocol import GreetingServiceProtocol


class GreetingService(GreetingServiceProtocol):
    """Generate the production greeting message."""

    def generate_greeting(self, name: str) -> str:
        return f"Hello, {name}"


_: GreetingServiceProtocol = GreetingService()
