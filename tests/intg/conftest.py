import pytest


@pytest.fixture(autouse=True)
def mock_env_for_integration_tests(monkeypatch):
    """Set up environment variables for all integration tests."""
    # Use mock implementations to avoid real network calls
    monkeypatch.setenv("TYP_TMPL_USE_MOCK_GREETING", "true")
