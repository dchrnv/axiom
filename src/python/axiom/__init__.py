"""
axiom — Python Library

High-performance semantic knowledge graph system built on axiom-core (Rust FFI).

Version: 0.1.0
License: AGPL-3.0-or-later (dual licensing available)
"""

__version__ = "0.1.0"
__author__ = "Axiom Team"
__license__ = "AGPL-3.0-or-later"

# Public API exports
from axiom.exceptions import (
    AxiomError,
    BootstrapError,
    ConfigError,
    QueryError,
    RuntimeError,
)
from axiom.query import QueryContext, QueryResult
from axiom.runtime import Config, Runtime
from axiom.runtime_storage import (
    RuntimeCDNAStorage,
    RuntimeConnectionStorage,
    RuntimeGridStorage,
    RuntimeTokenStorage,
)
from axiom.types import EmbeddingFormat, FeedbackType

__all__ = [
    # Core
    "Runtime",
    "Config",
    # Query
    "QueryResult",
    "QueryContext",
    # Exceptions
    "AxiomError",
    "RuntimeError",
    "QueryError",
    "BootstrapError",
    "ConfigError",
    # Types
    "FeedbackType",
    "EmbeddingFormat",
    # Runtime Storage
    "RuntimeTokenStorage",
    "RuntimeConnectionStorage",
    "RuntimeGridStorage",
    "RuntimeCDNAStorage",
]
