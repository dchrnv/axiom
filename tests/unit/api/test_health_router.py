"""
Unit tests for Health Check Router.

Tests Kubernetes health probes: /health, /health/live, /health/ready, /health/startup.
"""

import pytest
import time
from unittest.mock import Mock, patch
from fastapi import FastAPI
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

import api.routers.health as health
health_router = health.router


class TestBasicHealthCheck:
    """Test /health endpoint."""

    def test_health_check_returns_200(self):
        """Test that health check returns 200 status."""
        app = FastAPI()

        # Mock dependencies
        app.dependency_overrides[health.get_runtime] = lambda: Mock()

        def mock_token_storage():
            storage = Mock()
            storage.count.return_value = 0
            return storage

        app.dependency_overrides[health.get_token_storage] = mock_token_storage
        app.include_router(health_router)

        client = TestClient(app)
        response = client.get("/health")

        assert response.status_code == 200

    def test_health_check_includes_uptime(self):
        """Test that health check includes uptime."""
        app = FastAPI()

        # Mock dependencies
        app.dependency_overrides[health.get_runtime] = lambda: Mock()

        def mock_token_storage():
            storage = Mock()
            storage.count.return_value = 0
            return storage

        app.dependency_overrides[health.get_token_storage] = mock_token_storage
        app.include_router(health_router)

        client = TestClient(app)
        response = client.get("/health")

        data = response.json()
        assert "data" in data
        assert "uptime_seconds" in data["data"]
        assert isinstance(data["data"]["uptime_seconds"], (int, float))
        assert data["data"]["uptime_seconds"] >= 0

    def test_health_check_includes_version(self):
        """Test that health check includes version."""
        app = FastAPI()

        # Mock dependencies
        app.dependency_overrides[health.get_runtime] = lambda: Mock()

        def mock_token_storage():
            storage = Mock()
            storage.count.return_value = 0
            return storage

        app.dependency_overrides[health.get_token_storage] = mock_token_storage
        app.include_router(health_router)

        client = TestClient(app)
        response = client.get("/health")

        data = response.json()
        assert "version" in data["data"]
        assert isinstance(data["data"]["version"], str)

    def test_health_check_includes_runtime_metrics(self):
        """Test that health check includes runtime metrics."""
        app = FastAPI()

        # Mock dependencies
        app.dependency_overrides[health.get_runtime] = lambda: Mock()

        def mock_token_storage():
            storage = Mock()
            storage.count.return_value = 0
            return storage

        app.dependency_overrides[health.get_token_storage] = mock_token_storage
        app.include_router(health_router)

        client = TestClient(app)
        response = client.get("/health")

        data = response.json()
        assert "runtime_metrics" in data["data"]
        assert "tokens_count" in data["data"]["runtime_metrics"]
        assert "storage_backend" in data["data"]["runtime_metrics"]

    def test_health_check_status_healthy(self):
        """Test that health check returns healthy status."""
        app = FastAPI()

        # Mock dependencies
        app.dependency_overrides[health.get_runtime] = lambda: Mock()

        def mock_token_storage():
            storage = Mock()
            storage.count.return_value = 0
            return storage

        app.dependency_overrides[health.get_token_storage] = mock_token_storage
        app.include_router(health_router)

        client = TestClient(app)
        response = client.get("/health")

        data = response.json()
        assert data["success"] is True
        assert data["data"]["status"] in ["healthy", "degraded"]

    def test_health_check_handles_storage_errors(self):
        """Test that health check handles storage errors gracefully."""
        app = FastAPI()

        # Mock runtime
        app.dependency_overrides[health.get_runtime] = lambda: Mock()

        # Mock token storage to raise exception
        def mock_get_token_storage():
            storage = Mock()
            storage.count.side_effect = Exception("Storage unavailable")
            return storage

        app.dependency_overrides[health.get_token_storage] = mock_get_token_storage
        app.include_router(health_router)

        client = TestClient(app)
        response = client.get("/health")

        # Should still return 200 but with degraded status
        assert response.status_code == 200
        data = response.json()
        assert data["data"]["status"] == "degraded"


class TestLivenessProbe:
    """Test /health/live endpoint."""

    def test_liveness_returns_200(self):
        """Test that liveness probe returns 200."""
        app = FastAPI()
        app.include_router(health_router)

        client = TestClient(app)
        response = client.get("/health/live")

        assert response.status_code == 200

    def test_liveness_minimal_response(self):
        """Test that liveness returns minimal response."""
        app = FastAPI()
        app.include_router(health_router)

        client = TestClient(app)
        response = client.get("/health/live")

        data = response.json()
        assert "status" in data
        assert data["status"] == "alive"
        assert "check" in data
        assert data["check"] == "liveness"

    def test_liveness_always_succeeds(self):
        """Test that liveness probe always succeeds if process is running."""
        app = FastAPI()
        app.include_router(health_router)

        client = TestClient(app)

        # Make multiple requests
        for _ in range(5):
            response = client.get("/health/live")
            assert response.status_code == 200
            assert response.json()["status"] == "alive"


class TestReadinessProbe:
    """Test /health/ready endpoint."""

    def test_readiness_not_ready_during_startup(self):
        """Test that readiness returns 503 during startup period."""
        # Reset startup time to now
        with patch.object(health, '_start_time', time.time()):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock dependencies to prevent HTTPException
                app.dependency_overrides[health.get_runtime] = lambda: Mock()

                def mock_token_storage():
                    storage = Mock()
                    storage.count.return_value = 0
                    return storage

                def mock_grid_storage():
                    storage = Mock()
                    storage.get_grid.return_value = {"id": 0}
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage
                app.dependency_overrides[health.get_grid_storage] = mock_grid_storage
                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/ready")

                # Should be 503 if within minimum startup time
                assert response.status_code == 503
                data = response.json()
                # Response structure is different for early startup
                assert data.get("ready") is False or data.get("data", {}).get("ready") is False

    def test_readiness_checks_runtime(self):
        """Test that readiness checks runtime availability."""
        # Simulate after startup time
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock runtime as None (not available)
                app.dependency_overrides[health.get_runtime] = lambda: None

                # Mock storages as working
                def mock_token_storage():
                    storage = Mock()
                    storage.count.return_value = 0
                    return storage

                def mock_grid_storage():
                    storage = Mock()
                    storage.get_grid.return_value = {"id": 0}
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage
                app.dependency_overrides[health.get_grid_storage] = mock_grid_storage

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/ready")

                # Should fail if runtime not available
                assert response.status_code == 503

    def test_readiness_checks_token_storage(self):
        """Test that readiness checks token storage."""
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock token storage to fail
                def mock_token_storage():
                    storage = Mock()
                    storage.count.side_effect = Exception("Storage error")
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/ready")

                assert response.status_code == 503

    def test_readiness_checks_grid_storage(self):
        """Test that readiness checks grid storage."""
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock grid storage to fail
                def mock_grid_storage():
                    storage = Mock()
                    storage.get_grid.side_effect = Exception("Grid error")
                    return storage

                app.dependency_overrides[health.get_grid_storage] = mock_grid_storage

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/ready")

                assert response.status_code == 503

    def test_readiness_returns_200_when_ready(self):
        """Test that readiness returns 200 when all checks pass."""
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock all dependencies as working
                app.dependency_overrides[health.get_runtime] = lambda: Mock()

                def mock_token_storage():
                    storage = Mock()
                    storage.count.return_value = 5
                    return storage

                def mock_grid_storage():
                    storage = Mock()
                    storage.get_grid.return_value = {"id": 0}
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage
                app.dependency_overrides[health.get_grid_storage] = mock_grid_storage

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/ready")

                assert response.status_code == 200
                data = response.json()
                assert data["data"]["ready"] is True
                assert "checks" in data["data"]

    def test_readiness_marks_startup_complete(self):
        """Test that readiness marks startup as complete when ready."""
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock all dependencies as working
                app.dependency_overrides[health.get_runtime] = lambda: Mock()

                def mock_token_storage():
                    storage = Mock()
                    storage.count.return_value = 0
                    return storage

                def mock_grid_storage():
                    storage = Mock()
                    storage.get_grid.return_value = {"id": 0}
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage
                app.dependency_overrides[health.get_grid_storage] = mock_grid_storage

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/ready")

                # After successful ready check, _startup_complete should be True
                assert health._startup_complete is True


class TestStartupProbe:
    """Test /health/startup endpoint."""

    def test_startup_returns_503_initially(self):
        """Test that startup returns 503 before minimum uptime."""
        with patch.object(health, '_start_time', time.time()):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock dependencies to prevent HTTPException
                app.dependency_overrides[health.get_runtime] = lambda: Mock()

                def mock_token_storage():
                    storage = Mock()
                    storage.count.return_value = 0
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage
                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/startup")

                assert response.status_code == 503
                data = response.json()
                assert data["started"] is False

    def test_startup_checks_runtime(self):
        """Test that startup checks runtime initialization."""
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock runtime as None
                app.dependency_overrides[health.get_runtime] = lambda: None

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/startup")

                assert response.status_code == 503

    def test_startup_checks_storage(self):
        """Test that startup checks storage accessibility."""
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock storage to fail
                def mock_token_storage():
                    storage = Mock()
                    storage.count.side_effect = Exception("Not ready")
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/startup")

                assert response.status_code == 503

    def test_startup_returns_200_when_complete(self):
        """Test that startup returns 200 when startup is complete."""
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock dependencies as ready
                app.dependency_overrides[health.get_runtime] = lambda: Mock()

                def mock_token_storage():
                    storage = Mock()
                    storage.count.return_value = 0
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/startup")

                assert response.status_code == 200
                data = response.json()
                assert data["started"] is True
                assert "uptime_seconds" in data
                assert "checks" in data

    def test_startup_includes_uptime(self):
        """Test that startup response includes uptime."""
        with patch.object(health, '_start_time', time.time() - 5):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                app.dependency_overrides[health.get_runtime] = lambda: Mock()

                def mock_token_storage():
                    storage = Mock()
                    storage.count.return_value = 0
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage

                app.include_router(health_router)

                client = TestClient(app)
                response = client.get("/health/startup")

                if response.status_code == 200:
                    data = response.json()
                    assert "uptime_seconds" in data
                    assert data["uptime_seconds"] >= 2.0


class TestHealthProbesIntegration:
    """Test health probes working together."""

    def test_probe_progression(self):
        """Test typical probe progression: startup -> liveness -> readiness."""
        with patch.object(health, '_start_time', time.time() - 10):
            with patch.object(health, '_startup_complete', False):
                app = FastAPI()

                # Mock all dependencies as working
                app.dependency_overrides[health.get_runtime] = lambda: Mock()

                def mock_token_storage():
                    storage = Mock()
                    storage.count.return_value = 10
                    return storage

                def mock_grid_storage():
                    storage = Mock()
                    storage.get_grid.return_value = {"id": 0}
                    return storage

                app.dependency_overrides[health.get_token_storage] = mock_token_storage
                app.dependency_overrides[health.get_grid_storage] = mock_grid_storage

                app.include_router(health_router)

                client = TestClient(app)

                # 1. Startup probe should succeed
                startup_resp = client.get("/health/startup")
                assert startup_resp.status_code == 200

                # 2. Liveness probe should succeed
                liveness_resp = client.get("/health/live")
                assert liveness_resp.status_code == 200

                # 3. Readiness probe should succeed
                readiness_resp = client.get("/health/ready")
                assert readiness_resp.status_code == 200

    def test_liveness_independent_of_readiness(self):
        """Test that liveness works even when readiness fails."""
        with patch.object(health, '_start_time', time.time() - 10):
            app = FastAPI()

            # Mock dependencies to make readiness fail
            def mock_token_storage():
                storage = Mock()
                storage.count.side_effect = Exception("Failed")
                return storage

            app.dependency_overrides[health.get_token_storage] = mock_token_storage

            app.include_router(health_router)

            client = TestClient(app)

            # Liveness should still work
            liveness_resp = client.get("/health/live")
            assert liveness_resp.status_code == 200

            # But readiness should fail
            readiness_resp = client.get("/health/ready")
            assert readiness_resp.status_code == 503
