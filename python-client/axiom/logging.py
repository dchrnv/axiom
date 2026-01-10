"""
Logging configuration for Axiom client.

Provides structured logging for debugging and monitoring.
"""

import logging
import sys
from typing import Optional


# Logger name
LOGGER_NAME = "axiom"


def get_logger(name: Optional[str] = None) -> logging.Logger:
    """
    Get configured logger.

    Args:
        name: Logger name (defaults to 'axiom')

    Returns:
        Configured logger instance

    Example:
        >>> from axiom.logging import get_logger
        >>> logger = get_logger()
        >>> logger.info("Making API request")
    """
    if name is None:
        name = LOGGER_NAME
    elif not name.startswith(LOGGER_NAME):
        name = f"{LOGGER_NAME}.{name}"

    return logging.getLogger(name)


def setup_logging(
    level: int = logging.WARNING,
    format_string: Optional[str] = None,
    handler: Optional[logging.Handler] = None,
) -> None:
    """
    Setup logging configuration for Axiom client.

    Args:
        level: Logging level (default: WARNING)
        format_string: Custom format string
        handler: Custom handler (default: StreamHandler to stderr)

    Example:
        >>> import logging
        >>> from axiom.logging import setup_logging
        >>>
        >>> # Enable debug logging
        >>> setup_logging(level=logging.DEBUG)
        >>>
        >>> # Custom format
        >>> setup_logging(
        ...     level=logging.INFO,
        ...     format_string="%(asctime)s - %(name)s - %(message)s"
        ... )
    """
    logger = get_logger()

    # Remove existing handlers
    logger.handlers.clear()

    # Default format
    if format_string is None:
        format_string = (
            "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
        )

    formatter = logging.Formatter(format_string)

    # Default handler (stderr)
    if handler is None:
        handler = logging.StreamHandler(sys.stderr)

    handler.setFormatter(formatter)
    logger.addHandler(handler)
    logger.setLevel(level)

    # Prevent propagation to root logger
    logger.propagate = False


def enable_debug_logging():
    """
    Enable debug logging for Axiom client.

    Convenient shortcut for debugging.

    Example:
        >>> from axiom.logging import enable_debug_logging
        >>> enable_debug_logging()
    """
    setup_logging(level=logging.DEBUG)


def disable_logging():
    """
    Disable logging for Axiom client.

    Example:
        >>> from axiom.logging import disable_logging
        >>> disable_logging()
    """
    logger = get_logger()
    logger.handlers.clear()
    logger.addHandler(logging.NullHandler())
    logger.setLevel(logging.CRITICAL + 1)


# Example usage
if __name__ == "__main__":
    # Test logging
    setup_logging(level=logging.DEBUG)

    logger = get_logger()
    logger.debug("Debug message")
    logger.info("Info message")
    logger.warning("Warning message")
    logger.error("Error message")

    # Test sublogger
    sub_logger = get_logger("client")
    sub_logger.info("Client message")
