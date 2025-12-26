import pytest


@pytest.fixture(autouse=True)
def setup_unit_test(monkeypatch):
    """Set up environment variables for all unit tests."""
    monkeypatch.setenv("TLS_USE_MOCK_LLM", "true")
