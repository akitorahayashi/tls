"""Unit tests for tls services."""

import tempfile
from pathlib import Path

import pytest
from mocks.llm import MockLlmClient

from tls.protocols.llm import Message
from tls.services.initializer import Initializer


class TestMockLlmClient:
    """Tests for the mock LLM client."""

    @pytest.mark.asyncio
    async def test_mock_returns_configured_response(self) -> None:
        """Mock client returns the configured response."""
        client = MockLlmClient(response="Test response")
        messages = [Message(role="user", content="Hello")]
        result = await client.chat("test-model", messages)
        assert result == "Test response"


class TestInitializer:
    """Tests for the project initializer."""

    def test_init_creates_directories(self) -> None:
        """Initializer creates required directories."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir) / "test-project"
            initializer = Initializer()
            report = initializer.execute(root)

            assert (root / "benchmarks").exists()
            assert (root / "reports").exists()
            assert (root / "telescope.ini").exists()
            assert len(report.created_paths) > 0

    def test_init_creates_benchmark_files(self) -> None:
        """Initializer creates example benchmark files."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir) / "test-project"
            initializer = Initializer()
            initializer.execute(root)

            assert (root / "benchmarks" / "structured_output.json").exists()
            assert (root / "benchmarks" / "reasoning.json").exists()

    def test_init_creates_gitignore(self) -> None:
        """Initializer creates .gitignore with reports entry."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir) / "test-project"
            initializer = Initializer()
            initializer.execute(root)

            gitignore = root / ".gitignore"
            assert gitignore.exists()
            assert "reports/" in gitignore.read_text()

    def test_init_is_idempotent(self) -> None:
        """Running init twice doesn't duplicate content."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir) / "test-project"
            initializer = Initializer()

            # Run twice
            initializer.execute(root)
            report2 = initializer.execute(root)

            # Second run should create fewer items
            assert len(report2.created_paths) == 0
