"""LLM client service for API communication."""

from abc import ABC, abstractmethod
from dataclasses import dataclass

import httpx

from tls.core.exceptions import ConfigError, NetworkError


@dataclass
class Message:
    """Chat message structure."""

    role: str
    content: str

    def to_dict(self) -> dict[str, str]:
        """Convert to dictionary for JSON serialization."""
        return {"role": self.role, "content": self.content}


class GenAiClient(ABC):
    """Abstract base class for LLM clients."""

    @abstractmethod
    async def chat(self, model: str, messages: list[Message]) -> str:
        """
        Send a chat completion request.

        Args:
            model: Model name to use.
            messages: List of messages for the conversation.

        Returns:
            The model's response content.
        """
        pass


class LlmClient(GenAiClient):
    """HTTP client for OpenAI-compatible LLM APIs."""

    def __init__(
        self,
        base_url: str,
        api_key: str | None = None,
        timeout: int = 300,
    ) -> None:
        """
        Initialize the LLM client.

        Args:
            base_url: Base URL for the API endpoint.
            api_key: Optional API key for authentication.
            timeout: Request timeout in seconds.
        """
        self.api_key = api_key or "dummy"

        # Normalize URL to ensure trailing slash
        url = base_url.rstrip("/") + "/"
        try:
            self.base_url = url
        except Exception as e:
            raise ConfigError(f"Invalid base URL: {e}") from e

        self.timeout = timeout

    async def chat(self, model: str, messages: list[Message]) -> str:
        """
        Send a chat completion request.

        Args:
            model: Model name to use.
            messages: List of messages for the conversation.

        Returns:
            The model's response content.

        Raises:
            NetworkError: If the request fails.
        """
        url = f"{self.base_url}v1/chat/completions"
        headers = {"Authorization": f"Bearer {self.api_key}"}
        payload = {
            "model": model,
            "messages": [m.to_dict() for m in messages],
        }

        async with httpx.AsyncClient(timeout=self.timeout) as client:
            try:
                response = await client.post(url, json=payload, headers=headers)
            except httpx.RequestError as e:
                raise NetworkError(f"Request failed: {e}") from e

            if not response.is_success:
                raise NetworkError(
                    f"API Request failed: {response.status_code} - {response.text}"
                )

            try:
                data = response.json()
            except Exception as e:
                raise NetworkError(f"Failed to parse response: {e}") from e

            choices = data.get("choices", [])
            if not choices:
                raise NetworkError("No choices in response")

            content: str = choices[0]["message"]["content"]
            return content


class MockLlmClient(GenAiClient):
    """Mock LLM client for testing."""

    def __init__(self, response: str = "Mock response") -> None:
        """Initialize with a fixed response."""
        self.response = response

    async def chat(self, model: str, messages: list[Message]) -> str:
        """Return the configured mock response."""
        return self.response
