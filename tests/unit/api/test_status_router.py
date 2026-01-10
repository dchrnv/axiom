"""
Unit tests for Status Router.

Tests system status and statistics endpoints.
"""

import pytest
from unittest.mock import Mock, patch
from fastapi import FastAPI
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.auth.dependencies import get_current_active_user
import api.routers.status as status
status_router = status.router


class TestGetStatus:
    """Test GET /status endpoint."""

    def test_get_status_success(self):
        """Test successful status retrieval."""
        app = FastAPI()

        # Mock dependencies
        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_token_storage.count.return_value = 100
        mock_cdna_storage = Mock()
        mock_cdna_storage.get_config.return_value = {"profile_id": "test_profile"}

        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        with patch('api.routers.status.get_cached_system_metrics', return_value=(15.5, 256.0)):
            client = TestClient(app)
            response = client.get("/api/v1/status")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert "data" in data

            status_data = data["data"]
            assert status_data["state"] == "running"
            assert "uptime_seconds" in status_data
            assert status_data["tokens"]["total"] == 100
            assert status_data["memory_usage_mb"] == 256.0
            assert status_data["cpu_usage_percent"] == 15.5
            assert status_data["cdna_profile"] == "test_profile"
            assert status_data["storage_backend"] == "runtime"
            assert status_data["version"] == "0.52.0"

    def test_get_status_without_permission(self):
        """Test status fails without permission."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_cdna_storage = Mock()

        mock_user = Mock()
        mock_user.scopes = []  # No permissions

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/status")

        assert response.status_code == 403
        assert "status:read" in response.json()["detail"]

    def test_get_status_includes_components(self):
        """Test status includes component status."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_token_storage.count.return_value = 50
        mock_cdna_storage = Mock()
        mock_cdna_storage.get_config.return_value = {}

        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        with patch('api.routers.status.get_cached_system_metrics', return_value=(10.0, 128.0)):
            client = TestClient(app)
            response = client.get("/api/v1/status")

            assert response.status_code == 200
            data = response.json()
            components = data["data"]["components"]

            assert "runtime" in components
            assert "runtime_storage" in components
            assert "token_storage" in components
            assert "grid_storage" in components
            assert "cdna_storage" in components
            assert components["runtime"] == "running"

    def test_get_status_uptime_measured(self):
        """Test status includes uptime measurement."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_token_storage.count.return_value = 0
        mock_cdna_storage = Mock()
        mock_cdna_storage.get_config.return_value = {}

        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        with patch('api.routers.status.get_cached_system_metrics', return_value=(0.0, 0.0)):
            client = TestClient(app)
            response = client.get("/api/v1/status")

            assert response.status_code == 200
            data = response.json()
            assert "uptime_seconds" in data["data"]
            assert data["data"]["uptime_seconds"] >= 0

    def test_get_status_uses_cached_metrics(self):
        """Test status uses cached system metrics."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_token_storage.count.return_value = 10
        mock_cdna_storage = Mock()
        mock_cdna_storage.get_config.return_value = {}

        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        # Mock returns specific values
        with patch('api.routers.status.get_cached_system_metrics', return_value=(42.5, 512.75)) as mock_metrics:
            client = TestClient(app)
            response = client.get("/api/v1/status")

            assert response.status_code == 200
            data = response.json()

            # Verify cached metrics were used
            mock_metrics.assert_called_once()
            assert data["data"]["cpu_usage_percent"] == 42.5
            assert data["data"]["memory_usage_mb"] == 512.75

    def test_get_status_error_handling(self):
        """Test status handles storage errors gracefully."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_token_storage.count.side_effect = Exception("Storage error")
        mock_cdna_storage = Mock()

        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/status")

        # Should still return 200 but with degraded state
        assert response.status_code == 200
        data = response.json()
        assert data["data"]["state"] == "degraded"
        assert "error" in data["data"]
        assert data["data"]["components"]["runtime"] == "error"

    def test_get_status_token_count_correct(self):
        """Test status reports correct token count."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_token_storage.count.return_value = 150
        mock_cdna_storage = Mock()
        mock_cdna_storage.get_config.return_value = {}

        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        with patch('api.routers.status.get_cached_system_metrics', return_value=(0.0, 0.0)):
            client = TestClient(app)
            response = client.get("/api/v1/status")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["tokens"]["total"] == 150
            assert data["data"]["tokens"]["active"] == 150


class TestGetStats:
    """Test GET /stats endpoint."""

    def test_get_stats_success(self):
        """Test successful stats retrieval."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/stats")

        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert "data" in data

        stats_data = data["data"]
        assert "queries" in stats_data
        assert "feedbacks" in stats_data
        assert "cache" in stats_data
        assert "intuition" in stats_data

    def test_get_stats_without_permission(self):
        """Test stats fails without permission."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = []  # No permissions

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/stats")

        assert response.status_code == 403
        assert "status:read" in response.json()["detail"]

    def test_get_stats_includes_queries(self):
        """Test stats includes query statistics."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/stats")

        assert response.status_code == 200
        data = response.json()
        queries = data["data"]["queries"]

        assert "total" in queries
        assert "per_second" in queries
        assert "avg_latency_ms" in queries

    def test_get_stats_includes_feedbacks(self):
        """Test stats includes feedback statistics."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/stats")

        assert response.status_code == 200
        data = response.json()
        feedbacks = data["data"]["feedbacks"]

        assert "total" in feedbacks
        assert "positive" in feedbacks
        assert "negative" in feedbacks
        assert "corrections" in feedbacks

    def test_get_stats_includes_cache(self):
        """Test stats includes cache statistics."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/stats")

        assert response.status_code == 200
        data = response.json()
        cache = data["data"]["cache"]

        assert "hits" in cache
        assert "misses" in cache
        assert "hit_rate" in cache

    def test_get_stats_includes_intuition(self):
        """Test stats includes intuition engine statistics."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/stats")

        assert response.status_code == 200
        data = response.json()
        intuition = data["data"]["intuition"]

        assert "fast_path_hits" in intuition
        assert "slow_path_hits" in intuition
        assert "fast_path_rate" in intuition


class TestCachedSystemMetrics:
    """Test get_cached_system_metrics function."""

    def test_cached_metrics_returns_tuple(self):
        """Test cached metrics returns CPU and memory tuple."""
        # Reset cache
        status._last_cache_update = 0.0

        with patch('api.routers.status.psutil.Process') as mock_process_class:
            mock_process = Mock()
            mock_process.cpu_percent.return_value = 25.0
            mock_process.memory_info.return_value.rss = 512 * 1024 * 1024  # 512 MB in bytes
            mock_process_class.return_value = mock_process

            cpu, memory = status.get_cached_system_metrics()

            assert isinstance(cpu, float)
            assert isinstance(memory, float)
            assert cpu == 25.0
            assert memory == 512.0

    def test_cached_metrics_uses_cache(self):
        """Test cached metrics uses cache within TTL."""
        # Set recent cache values
        status._cached_cpu_percent = 30.0
        status._cached_memory_mb = 256.0
        status._last_cache_update = status.time.time()

        with patch('api.routers.status.psutil.Process') as mock_process_class:
            # This should not be called since cache is valid
            cpu, memory = status.get_cached_system_metrics()

            # Should return cached values
            assert cpu == 30.0
            assert memory == 256.0
            # psutil should not be called
            mock_process_class.assert_not_called()

    def test_cached_metrics_updates_after_ttl(self):
        """Test cached metrics updates after TTL expires."""
        # Set old cache
        status._cached_cpu_percent = 10.0
        status._cached_memory_mb = 100.0
        status._last_cache_update = 0.0  # Very old
        status._process = None

        with patch('api.routers.status.psutil.Process') as mock_process_class:
            mock_process = Mock()
            mock_process.cpu_percent.return_value = 50.0
            mock_process.memory_info.return_value.rss = 1024 * 1024 * 1024  # 1 GB
            mock_process_class.return_value = mock_process

            cpu, memory = status.get_cached_system_metrics()

            # Should return new values
            assert cpu == 50.0
            assert memory == 1024.0
            mock_process_class.assert_called_once()

    def test_cached_metrics_handles_errors(self):
        """Test cached metrics handles psutil errors gracefully."""
        # Set old values
        status._cached_cpu_percent = 20.0
        status._cached_memory_mb = 200.0
        status._last_cache_update = 0.0
        status._process = None

        with patch('api.routers.status.psutil.Process', side_effect=Exception("psutil error")):
            cpu, memory = status.get_cached_system_metrics()

            # Should return old cached values on error
            assert cpu == 20.0
            assert memory == 200.0


class TestStatusIntegration:
    """Integration tests for status endpoints."""

    def test_status_and_stats_both_work(self):
        """Test both status and stats endpoints work together."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_token_storage.count.return_value = 75
        mock_cdna_storage = Mock()
        mock_cdna_storage.get_config.return_value = {"profile_id": "prod"}

        mock_user = Mock()
        mock_user.scopes = ["status:read"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        with patch('api.routers.status.get_cached_system_metrics', return_value=(5.0, 128.0)):
            client = TestClient(app)

            # Get status
            status_response = client.get("/api/v1/status")
            assert status_response.status_code == 200

            # Get stats
            stats_response = client.get("/api/v1/stats")
            assert stats_response.status_code == 200

            # Both should succeed
            assert status_response.json()["success"] is True
            assert stats_response.json()["success"] is True

    def test_permission_required_for_both_endpoints(self):
        """Test both endpoints require status:read permission."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_token_storage = Mock()
        mock_cdna_storage = Mock()

        mock_user = Mock()
        mock_user.scopes = ["some:other:permission"]

        app.dependency_overrides[status.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[status.get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[status.get_cdna_storage] = lambda: mock_cdna_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(status_router, prefix="/api/v1")

        client = TestClient(app)

        # Both should fail without permission
        status_response = client.get("/api/v1/status")
        stats_response = client.get("/api/v1/stats")

        assert status_response.status_code == 403
        assert stats_response.status_code == 403
