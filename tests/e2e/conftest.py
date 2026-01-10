"""
E2E tests configuration and fixtures.

These tests require Rust Core to be built and available.
"""

import pytest

# Mark all tests in this directory as requiring Rust Core
pytestmark = pytest.mark.requires_rust_core


@pytest.fixture(scope="session")
def rust_core_available():
    """Check if Rust Core is available."""
    try:
        import axiom._core
        return True
    except (ImportError, AttributeError):
        pytest.skip("Rust Core not available - skipping E2E tests")
        return False
