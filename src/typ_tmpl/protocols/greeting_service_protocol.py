from typing import Protocol


class GreetingServiceProtocol(Protocol):
    def generate_greeting(self, name: str) -> str: ...
