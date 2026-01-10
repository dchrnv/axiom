"""
Axiom Python Client

Official Python client library for Axiom semantic knowledge system.

Basic Usage:
    >>> from axiom import AxiomClient
    >>>
    >>> # Using JWT authentication
    >>> client = AxiomClient(
    ...     base_url="http://localhost:8000",
    ...     username="developer",
    ...     password="developer123"
    ... )
    >>>
    >>> # Create a token
    >>> token = client.tokens.create(text="hello world")
    >>> print(token.id, token.embedding)
    >>>
    >>> # Query tokens
    >>> results = client.tokens.query(
    ...     query_vector=[0.1, 0.2, ...],
    ...     top_k=10
    ... )

Async Usage:
    >>> from axiom import AsyncAxiomClient
    >>>
    >>> async with AsyncAxiomClient(
    ...     base_url="http://localhost:8000",
    ...     api_key="your-api-key"
    ... ) as client:
    ...     token = await client.tokens.create(text="hello world")
    ...     results = await client.tokens.query(...)
"""

from .client import AxiomClient
from .async_client import AsyncAxiomClient
from .exceptions import (
    AxiomError,
    AuthenticationError,
    AuthorizationError,
    NotFoundError,
    ValidationError,
    RateLimitError,
    ServerError,
)
from .models import (
    Token,
    TokenCreate,
    TokenQuery,
    APIKey,
    APIKeyCreate,
    User,
)
from .retry import RetryConfig, retry_with_backoff, async_retry_with_backoff
from .logging import setup_logging, get_logger, enable_debug_logging

__version__ = "0.59.0"
__author__ = "Chernov Denys"
__license__ = "AGPLv3"

__all__ = [
    # Clients
    "AxiomClient",
    "AsyncAxiomClient",
    # Exceptions
    "AxiomError",
    "AuthenticationError",
    "AuthorizationError",
    "NotFoundError",
    "ValidationError",
    "RateLimitError",
    "ServerError",
    # Models
    "Token",
    "TokenCreate",
    "TokenQuery",
    "APIKey",
    "APIKeyCreate",
    "User",
    # Retry
    "RetryConfig",
    "retry_with_backoff",
    "async_retry_with_backoff",
    # Logging
    "setup_logging",
    "get_logger",
    "enable_debug_logging",
]
