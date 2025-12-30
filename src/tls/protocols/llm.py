"""Protocol for LLM client implementations."""

from dataclasses import dataclass
from typing import Protocol


@dataclass
class Message:
    """Chat message structure."""

    role: str
    content: str

    def to_dict(self) -> dict[str, str]:
        """Convert to dictionary for JSON serialization."""
        return {"role": self.role, "content": self.content}


class LlmClientProtocol(Protocol):
    """Protocol for LLM client implementations."""

    async def chat(self, model: str, messages: list[Message]) -> str:
        """
        Send a chat completion request.

        Args:
            model: Model name to use.
            messages: List of messages for the conversation.

        Returns:
            The model's response content.
        """
        ...
