"""Custom exceptions for tls application."""


class TlsError(Exception):
    """Base exception for all tls errors."""

    pass


class ConfigError(TlsError):
    """Configuration-related errors."""

    pass


class NetworkError(TlsError):
    """Network/HTTP-related errors."""

    pass


class ValidationError(TlsError):
    """Data validation errors."""

    pass
