from typing import Generator

import pytest


@pytest.fixture(autouse=True)
def mock_env_for_integration_tests(
    monkeypatch: pytest.MonkeyPatch,
) -> Generator[None, None, None]:
    """Set up environment variables for all integration tests."""
    # Use mock implementations to avoid real network calls
    monkeypatch.setenv("TLS_USE_MOCK_LLM", "true")
    yield
