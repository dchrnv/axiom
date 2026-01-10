"""
Unit tests for Cache Stats Router.

Tests cache statistics and cleanup endpoints.
"""

import pytest
from unittest.mock import Mock, patch
from fastapi import FastAPI
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

import api.routers.cache_stats as cache_stats
cache_stats_router = cache_stats.router


class TestGetCacheStats:
    """Test GET /cache/stats endpoint."""

    def test_get_stats_success(self):
        """Test successful cache stats retrieval."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        # Mock cache stats
        mock_stats = {
            "token_cache": {
                "size": 100,
                "max_size": 1000,
                "hits": 500,
                "misses": 50,
                "hit_rate": 90.9,
                "evictions": 10
            },
            "query_cache": {
                "size": 200,
                "max_size": 500,
                "hits": 1000,
                "misses": 100,
                "hit_rate": 90.9,
                "evictions": 5
            }
        }

        with patch('api.routers.cache_stats.get_all_cache_stats', return_value=mock_stats):
            # Mock authentication
            mock_user = Mock()
            mock_user.username = "admin"
            app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

            client = TestClient(app)
            response = client.get("/api/v1/cache/stats")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert "cache statistics" in data["message"].lower()
            assert "data" in data
            assert "token_cache" in data["data"]
            assert "query_cache" in data["data"]

    def test_get_stats_includes_all_metrics(self):
        """Test that stats include all cache metrics."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        mock_stats = {
            "test_cache": {
                "size": 50,
                "max_size": 100,
                "hits": 200,
                "misses": 20,
                "hit_rate": 90.9,
                "evictions": 2
            }
        }

        with patch('api.routers.cache_stats.get_all_cache_stats', return_value=mock_stats):
            mock_user = Mock()
            mock_user.username = "admin"
            app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

            client = TestClient(app)
            response = client.get("/api/v1/cache/stats")

            assert response.status_code == 200
            data = response.json()
            cache = data["data"]["test_cache"]

            # Verify all metrics present
            assert "size" in cache
            assert "max_size" in cache
            assert "hits" in cache
            assert "misses" in cache
            assert "hit_rate" in cache
            assert "evictions" in cache

    def test_get_stats_without_auth(self):
        """Test that stats require authentication."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/cache/stats")

        # Should fail without authentication
        assert response.status_code in [401, 403]

    def test_get_stats_empty_caches(self):
        """Test stats with no caches."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        with patch('api.routers.cache_stats.get_all_cache_stats', return_value={}):
            mock_user = Mock()
            mock_user.username = "admin"
            app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

            client = TestClient(app)
            response = client.get("/api/v1/cache/stats")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert data["data"] == {}

    def test_get_stats_with_different_users(self):
        """Test that authenticated users can get stats."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        mock_stats = {"cache": {"size": 10}}

        with patch('api.routers.cache_stats.get_all_cache_stats', return_value=mock_stats):
            # Test with admin user
            mock_admin = Mock()
            mock_admin.username = "admin"
            app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_admin

            client = TestClient(app)
            response = client.get("/api/v1/cache/stats")
            assert response.status_code == 200

    def test_get_stats_returns_success_response(self):
        """Test that response follows SuccessResponse format."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        mock_stats = {"cache": {"size": 5}}

        with patch('api.routers.cache_stats.get_all_cache_stats', return_value=mock_stats):
            mock_user = Mock()
            mock_user.username = "admin"
            app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

            client = TestClient(app)
            response = client.get("/api/v1/cache/stats")

            data = response.json()
            # Check SuccessResponse structure
            assert "success" in data
            assert "message" in data
            assert "data" in data
            assert data["success"] is True


class TestCleanupCaches:
    """Test POST /cache/cleanup endpoint."""

    def test_cleanup_success(self):
        """Test successful cache cleanup."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        mock_stats_after = {
            "cache": {"size": 50, "evictions": 10}
        }

        with patch('api.routers.cache_stats.cleanup_all_caches') as mock_cleanup:
            with patch('api.routers.cache_stats.get_all_cache_stats', return_value=mock_stats_after):
                mock_user = Mock()
                mock_user.username = "admin"
                app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

                client = TestClient(app)
                response = client.post("/api/v1/cache/cleanup")

                assert response.status_code == 200
                data = response.json()
                assert data["success"] is True
                assert "cleanup" in data["message"].lower()
                mock_cleanup.assert_called_once()

    def test_cleanup_returns_updated_stats(self):
        """Test that cleanup returns updated cache stats."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        mock_stats = {
            "cache": {
                "size": 30,
                "max_size": 100,
                "hits": 100,
                "misses": 10,
                "hit_rate": 90.9,
                "evictions": 20
            }
        }

        with patch('api.routers.cache_stats.cleanup_all_caches'):
            with patch('api.routers.cache_stats.get_all_cache_stats', return_value=mock_stats):
                mock_user = Mock()
                mock_user.username = "admin"
                app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

                client = TestClient(app)
                response = client.post("/api/v1/cache/cleanup")

                assert response.status_code == 200
                data = response.json()
                assert "data" in data
                assert data["data"]["cache"]["size"] == 30
                assert data["data"]["cache"]["evictions"] == 20

    def test_cleanup_without_auth(self):
        """Test that cleanup requires authentication."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post("/api/v1/cache/cleanup")

        # Should fail without authentication
        assert response.status_code in [401, 403]

    def test_cleanup_calls_cleanup_function(self):
        """Test that cleanup actually calls the cleanup function."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        with patch('api.routers.cache_stats.cleanup_all_caches') as mock_cleanup:
            with patch('api.routers.cache_stats.get_all_cache_stats', return_value={}):
                mock_user = Mock()
                mock_user.username = "admin"
                app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

                client = TestClient(app)
                response = client.post("/api/v1/cache/cleanup")

                assert response.status_code == 200
                # Verify cleanup was called
                mock_cleanup.assert_called_once()

    def test_cleanup_returns_success_response(self):
        """Test that cleanup response follows SuccessResponse format."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        with patch('api.routers.cache_stats.cleanup_all_caches'):
            with patch('api.routers.cache_stats.get_all_cache_stats', return_value={}):
                mock_user = Mock()
                mock_user.username = "admin"
                app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

                client = TestClient(app)
                response = client.post("/api/v1/cache/cleanup")

                data = response.json()
                # Check SuccessResponse structure
                assert "success" in data
                assert "message" in data
                assert "data" in data
                assert data["success"] is True

    def test_cleanup_with_different_users(self):
        """Test that authenticated users can cleanup caches."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        with patch('api.routers.cache_stats.cleanup_all_caches'):
            with patch('api.routers.cache_stats.get_all_cache_stats', return_value={}):
                # Test with developer user
                mock_dev = Mock()
                mock_dev.username = "developer"
                app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_dev

                client = TestClient(app)
                response = client.post("/api/v1/cache/cleanup")
                assert response.status_code == 200


class TestCacheStatsIntegration:
    """Test cache stats endpoints working together."""

    def test_stats_then_cleanup(self):
        """Test getting stats then cleaning up."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        stats_before = {"cache": {"size": 100, "evictions": 5}}
        stats_after = {"cache": {"size": 80, "evictions": 25}}

        mock_user = Mock()
        mock_user.username = "admin"
        app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

        client = TestClient(app)

        # 1. Get stats before cleanup
        with patch('api.routers.cache_stats.get_all_cache_stats', return_value=stats_before):
            response1 = client.get("/api/v1/cache/stats")
            assert response1.status_code == 200
            data1 = response1.json()
            assert data1["data"]["cache"]["size"] == 100

        # 2. Cleanup caches
        with patch('api.routers.cache_stats.cleanup_all_caches'):
            with patch('api.routers.cache_stats.get_all_cache_stats', return_value=stats_after):
                response2 = client.post("/api/v1/cache/cleanup")
                assert response2.status_code == 200
                data2 = response2.json()
                assert data2["data"]["cache"]["size"] == 80
                assert data2["data"]["cache"]["evictions"] == 25

    def test_multiple_cleanup_calls(self):
        """Test that cleanup can be called multiple times."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        with patch('api.routers.cache_stats.cleanup_all_caches') as mock_cleanup:
            with patch('api.routers.cache_stats.get_all_cache_stats', return_value={}):
                mock_user = Mock()
                mock_user.username = "admin"
                app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

                client = TestClient(app)

                # Call cleanup 3 times
                for _ in range(3):
                    response = client.post("/api/v1/cache/cleanup")
                    assert response.status_code == 200

                # Should be called 3 times
                assert mock_cleanup.call_count == 3

    def test_stats_unchanged_by_get(self):
        """Test that getting stats doesn't change cache state."""
        app = FastAPI()
        app.include_router(cache_stats_router, prefix="/api/v1")

        mock_stats = {"cache": {"size": 50}}

        with patch('api.routers.cache_stats.get_all_cache_stats', return_value=mock_stats) as mock_get:
            mock_user = Mock()
            mock_user.username = "admin"
            app.dependency_overrides[cache_stats.get_current_active_user] = lambda: mock_user

            client = TestClient(app)

            # Get stats multiple times
            for _ in range(3):
                response = client.get("/api/v1/cache/stats")
                assert response.status_code == 200

            # get_all_cache_stats should be called 3 times
            assert mock_get.call_count == 3
