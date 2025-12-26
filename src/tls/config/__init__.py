"""Configuration module for tls."""

from tls.config.settings import AppSettings, load_config, settings
from tls.config.templates import (
    BENCHMARK_REASONING,
    BENCHMARK_STRUCTURED,
    DEFAULT_CONFIG,
    GITIGNORE_ENTRIES,
)

__all__ = [
    "AppSettings",
    "BENCHMARK_REASONING",
    "BENCHMARK_STRUCTURED",
    "DEFAULT_CONFIG",
    "GITIGNORE_ENTRIES",
    "load_config",
    "settings",
]
