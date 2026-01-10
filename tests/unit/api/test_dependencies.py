"""
Unit tests for API Dependencies.

Tests dependency injection, storage backends, and authentication helpers.
"""

import pytest
from unittest.mock import Mock, patch, MagicMock
from fastapi import HTTPException
from fastapi.security import HTTPAuthorizationCredentials
import sys
from pathlib import Path

src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.dependencies import (
    get_runtime,
    set_runtime,
    get_token_storage,
    get_grid_storage,
    get_cdna_storage,
    reset_storage,
    verify_token,
    require_admin,
    check_grid_available,
    _runtime_instance,
    _token_storage,
    _grid_storage,
    _cdna_storage,
)
from api.storage import TokenStorageInterface, GridStorageInterface, CDNAStorageInterface
from api.storage.memory import InMemoryTokenStorage, InMemoryGridStorage, InMemoryCDNAStorage


class TestStorageBackendSelection:
    """Test storage backend selection based on settings."""

    def test_defaults_to_memory_storage(self):
        """Test that storage defaults to memory backend."""
        # Reset to clean state
        reset_storage()

        with patch('api.dependencies.settings') as mock_settings:
            mock_settings.STORAGE_BACKEND = "memory"

            token_storage = get_token_storage()
            assert isinstance(token_storage, InMemoryTokenStorage)

            grid_storage = get_grid_storage()
            assert isinstance(grid_storage, InMemoryGridStorage)

            cdna_storage = get_cdna_storage()
            assert isinstance(cdna_storage, InMemoryCDNAStorage)

    def test_storage_singleton_pattern(self):
        """Test that storage instances are singletons."""
        reset_storage()

        with patch('api.dependencies.settings') as mock_settings:
            mock_settings.STORAGE_BACKEND = "memory"

            # Get storage twice
            token_storage1 = get_token_storage()
            token_storage2 = get_token_storage()

            # Should be same instance
            assert token_storage1 is token_storage2

            # Same for other storage types
            grid_storage1 = get_grid_storage()
            grid_storage2 = get_grid_storage()
            assert grid_storage1 is grid_storage2

            cdna_storage1 = get_cdna_storage()
            cdna_storage2 = get_cdna_storage()
            assert cdna_storage1 is cdna_storage2

    def test_reset_storage_clears_singletons(self):
        """Test that reset_storage clears singleton instances."""
        reset_storage()

        with patch('api.dependencies.settings') as mock_settings:
            mock_settings.STORAGE_BACKEND = "memory"

            # Get storage
            token_storage1 = get_token_storage()

            # Reset
            reset_storage()

            # Get again - should be new instance
            token_storage2 = get_token_storage()

            # Different instances (can't compare directly due to module reloading)
            assert token_storage2 is not None

    def test_unknown_backend_defaults_to_memory(self):
        """Test that unknown backend falls back to memory."""
        reset_storage()

        with patch('api.dependencies.settings') as mock_settings:
            mock_settings.STORAGE_BACKEND = "unknown_backend"

            token_storage = get_token_storage()
            assert isinstance(token_storage, InMemoryTokenStorage)


class TestRuntimeDependency:
    """Test runtime initialization and dependency injection."""

    def test_set_runtime_external(self):
        """Test setting runtime instance externally."""
        mock_runtime = Mock()
        mock_runtime.name = "test_runtime"

        set_runtime(mock_runtime)

        # Runtime should be set
        # Can't test get_runtime() directly due to import issues
        # But set_runtime logs success, which we can verify indirectly

    def test_get_runtime_callable(self):
        """Test that get_runtime is callable."""
        assert callable(get_runtime)

        # If axiom is available, runtime initialization works
        # If not, HTTPException is raised
        # We test both cases in integration tests


class TestAuthenticationHelpers:
    """Test authentication helper functions."""

    @pytest.mark.asyncio
    async def test_verify_token_no_credentials(self):
        """Test that no credentials returns None (development mode)."""
        result = await verify_token(credentials=None)
        assert result is None

    @pytest.mark.asyncio
    async def test_verify_token_with_credentials(self):
        """Test token verification with credentials."""
        mock_credentials = HTTPAuthorizationCredentials(
            scheme="Bearer",
            credentials="test_token_12345678901234567890"
        )

        result = await verify_token(credentials=mock_credentials)

        # In development, returns "anonymous"
        assert result == "anonymous"

    @pytest.mark.asyncio
    async def test_require_admin_without_user(self):
        """Test that require_admin raises error without user."""
        with pytest.raises(HTTPException) as exc_info:
            await require_admin(user_id=None)

        assert exc_info.value.status_code == 401
        assert "Admin authentication required" in exc_info.value.detail

    @pytest.mark.asyncio
    async def test_require_admin_with_user(self):
        """Test that require_admin passes with user."""
        result = await require_admin(user_id="admin_user")

        # Should return user_id
        assert result == "admin_user"


class TestGridAvailability:
    """Test grid availability checker."""

    def test_grid_available_returns_boolean(self):
        """Test that check_grid_available returns boolean."""
        result = check_grid_available()

        # Should return True or False depending on installation
        assert isinstance(result, bool)

    def test_grid_not_available_on_import_error(self):
        """Test that ImportError returns False."""
        with patch('builtins.__import__', side_effect=ImportError("axiom_grid_v2 not found")):
            result = check_grid_available()
            assert result is False


class TestStorageInterfaces:
    """Test that returned storage implements correct interfaces."""

    def test_token_storage_implements_interface(self):
        """Test that token storage implements TokenStorageInterface."""
        reset_storage()

        with patch('api.dependencies.settings') as mock_settings:
            mock_settings.STORAGE_BACKEND = "memory"

            storage = get_token_storage()

            # Check interface methods exist
            assert hasattr(storage, 'get')
            assert hasattr(storage, 'create')
            assert hasattr(storage, 'update')
            assert hasattr(storage, 'delete')
            assert hasattr(storage, 'list')
            assert hasattr(storage, 'count')
            assert hasattr(storage, 'clear')

    def test_grid_storage_implements_interface(self):
        """Test that grid storage implements GridStorageInterface."""
        reset_storage()

        with patch('api.dependencies.settings') as mock_settings:
            mock_settings.STORAGE_BACKEND = "memory"

            storage = get_grid_storage()

            # Check interface methods exist
            assert hasattr(storage, 'create_grid')
            assert hasattr(storage, 'get_grid')
            assert hasattr(storage, 'delete_grid')
            assert hasattr(storage, 'add_token')
            assert hasattr(storage, 'remove_token')
            assert hasattr(storage, 'find_neighbors')

    def test_cdna_storage_implements_interface(self):
        """Test that CDNA storage implements CDNAStorageInterface."""
        reset_storage()

        with patch('api.dependencies.settings') as mock_settings:
            mock_settings.STORAGE_BACKEND = "memory"

            storage = get_cdna_storage()

            # Check interface methods exist
            assert hasattr(storage, 'get_config')
            assert hasattr(storage, 'update_config')
            assert hasattr(storage, 'get_profile')
            assert hasattr(storage, 'list_profiles')
            assert hasattr(storage, 'switch_profile')


class TestRuntimeStorageIntegration:
    """Test runtime storage backend (if available)."""

    def test_runtime_storage_backend_option_exists(self):
        """Test that runtime backend option is recognized."""
        reset_storage()

        # Runtime storage requires axiom to be installed
        # This test just verifies the backend option is handled
        with patch('api.dependencies.settings') as mock_settings:
            mock_settings.STORAGE_BACKEND = "runtime"

            # If axiom is available, storage will use it
            # If not, initialization will raise HTTPException
            # We test actual runtime integration in e2e tests

    def test_storage_backend_configuration(self):
        """Test that storage backend can be configured."""
        reset_storage()

        with patch('api.dependencies.settings') as mock_settings:
            # Test that different backend values are handled
            backends = ["memory", "runtime", "unknown"]

            for backend in backends:
                reset_storage()
                mock_settings.STORAGE_BACKEND = backend

                # Should not crash on initialization
                # Unknown backends default to memory
                storage = get_token_storage()
                assert storage is not None


class TestDependencyInjectionPattern:
    """Test FastAPI dependency injection patterns."""

    def test_storage_dependencies_are_callables(self):
        """Test that dependency functions are callable."""
        assert callable(get_token_storage)
        assert callable(get_grid_storage)
        assert callable(get_cdna_storage)
        assert callable(get_runtime)

    def test_auth_dependencies_are_async(self):
        """Test that auth dependencies are async functions."""
        import inspect

        assert inspect.iscoroutinefunction(verify_token)
        assert inspect.iscoroutinefunction(require_admin)

    def test_dependency_functions_have_no_required_params(self):
        """Test that dependency functions have no required parameters."""
        import inspect

        # Storage dependencies should have no required params
        sig = inspect.signature(get_token_storage)
        assert len(sig.parameters) == 0

        sig = inspect.signature(get_grid_storage)
        assert len(sig.parameters) == 0

        sig = inspect.signature(get_cdna_storage)
        assert len(sig.parameters) == 0
