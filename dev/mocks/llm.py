"""Mock LLM client for testing."""

from tls.protocols.llm import Message


class MockLlmClient:
    """Mock LLM client for testing."""

    def __init__(self, response: str = "Mock response") -> None:
        """Initialize with a fixed response."""
        self.response = response

    async def chat(self, model: str, messages: list[Message]) -> str:
        """Return the configured mock response."""
        return self.response
