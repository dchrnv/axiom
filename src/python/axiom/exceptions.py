"""Exception classes for axiom library."""


class AxiomError(Exception):
    """Base exception for all axiom errors."""

    pass


class RuntimeError(AxiomError):
    """Raised when runtime initialization or operation fails."""

    pass


class QueryError(AxiomError):
    """Raised when query execution fails."""

    pass


class BootstrapError(AxiomError):
    """Raised when bootstrap loading fails."""

    pass


class ConfigError(AxiomError):
    """Raised when configuration is invalid."""

    pass


class FFIError(AxiomError):
    """Raised when FFI call to Rust core fails."""

    pass


class EmbeddingError(AxiomError):
    """Raised when embedding operations fail."""

    pass
