"""
Unit tests for Metrics Router.

Tests Prometheus and JSON metrics endpoints.
"""

import pytest
from unittest.mock import Mock, patch
from fastapi import FastAPI, Response
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.auth.dependencies import get_current_active_user
import api.routers.metrics as metrics
metrics_router = metrics.router


class TestGetPrometheusMetrics:
    """Test GET /metrics endpoint (Prometheus format)."""

    def test_get_metrics_success(self):
        """Test successful Prometheus metrics retrieval."""
        app = FastAPI()

        # Mock dependencies
        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 100
        mock_runtime.connections.count.return_value = 5

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        with patch('api.routers.metrics.update_system_metrics') as mock_update:
            with patch('api.routers.metrics.get_metrics_response', return_value=Response(content="# Metrics\n", media_type="text/plain")):
                client = TestClient(app)
                response = client.get("/api/v1/metrics")

                assert response.status_code == 200
                assert response.headers["content-type"] == "text/plain; charset=utf-8"
                assert "Metrics" in response.text

                # Verify update_system_metrics was called with correct values
                mock_update.assert_called_once()
                call_args = mock_update.call_args[1]
                assert call_args["token_count"] == 100
                assert call_args["connection_count"] == 5
                assert call_args["memory_bytes"] == 6400  # 100 * 64

    def test_get_metrics_without_permission(self):
        """Test metrics fails without permission."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = []  # No permissions

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics")

        assert response.status_code == 403
        assert "metrics:read" in response.json()["detail"]

    def test_get_metrics_updates_system_metrics(self):
        """Test metrics updates system metrics before returning."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 50
        mock_runtime.connections.count.return_value = 3

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        with patch('api.routers.metrics.update_system_metrics') as mock_update:
            with patch('api.routers.metrics.get_metrics_response', return_value=Response(content="", media_type="text/plain")):
                client = TestClient(app)
                response = client.get("/api/v1/metrics")

                assert response.status_code == 200
                mock_update.assert_called_once_with(
                    token_count=50,
                    connection_count=3,
                    memory_bytes=3200  # 50 * 64
                )

    def test_get_metrics_calculates_memory_estimate(self):
        """Test metrics calculates memory estimate from token count."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 1000
        mock_runtime.connections.count.return_value = 10

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        with patch('api.routers.metrics.update_system_metrics') as mock_update:
            with patch('api.routers.metrics.get_metrics_response', return_value=Response(content="", media_type="text/plain")):
                client = TestClient(app)
                response = client.get("/api/v1/metrics")

                assert response.status_code == 200
                # Memory should be token_count * 64
                call_args = mock_update.call_args[1]
                assert call_args["memory_bytes"] == 64000

    def test_get_metrics_error_handling(self):
        """Test metrics handles errors gracefully."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.side_effect = Exception("Runtime error")

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics")

        # Should return 200 with empty content (to not break Prometheus)
        assert response.status_code == 200
        assert response.headers["content-type"] == "text/plain; version=0.0.4; charset=utf-8"
        assert response.text == ""

    def test_get_metrics_prometheus_format(self):
        """Test metrics returns Prometheus text format."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 10
        mock_runtime.connections.count.return_value = 1

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        prometheus_content = "# TYPE tokens_total gauge\ntokens_total 10\n"
        with patch('api.routers.metrics.update_system_metrics'):
            with patch('api.routers.metrics.get_metrics_response', return_value=Response(content=prometheus_content, media_type="text/plain")):
                client = TestClient(app)
                response = client.get("/api/v1/metrics")

                assert response.status_code == 200
                assert "tokens_total" in response.text


class TestGetMetricsJson:
    """Test GET /metrics/json endpoint."""

    def test_get_metrics_json_success(self):
        """Test successful JSON metrics retrieval."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 75
        mock_runtime.connections.count.return_value = 4

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics/json")

        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert "data" in data

        metrics_data = data["data"]
        assert "system" in metrics_data
        assert "tokens" in metrics_data
        assert "connections" in metrics_data
        assert "storage" in metrics_data

    def test_get_metrics_json_without_permission(self):
        """Test JSON metrics fails without permission."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = []  # No permissions

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics/json")

        assert response.status_code == 403
        assert "metrics:read" in response.json()["detail"]

    def test_get_metrics_json_includes_system_info(self):
        """Test JSON metrics includes system information."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 10
        mock_runtime.connections.count.return_value = 1

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics/json")

        assert response.status_code == 200
        data = response.json()
        system = data["data"]["system"]

        assert "uptime_seconds" in system
        assert "version" in system
        assert system["version"] == "0.52.0"

    def test_get_metrics_json_includes_token_count(self):
        """Test JSON metrics includes token count."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 150
        mock_runtime.connections.count.return_value = 2

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics/json")

        assert response.status_code == 200
        data = response.json()
        tokens = data["data"]["tokens"]

        assert "active_count" in tokens
        assert tokens["active_count"] == 150

    def test_get_metrics_json_includes_connection_count(self):
        """Test JSON metrics includes connection count."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 10
        mock_runtime.connections.count.return_value = 8

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics/json")

        assert response.status_code == 200
        data = response.json()
        connections = data["data"]["connections"]

        assert "active_count" in connections
        assert connections["active_count"] == 8

    def test_get_metrics_json_includes_storage_info(self):
        """Test JSON metrics includes storage backend info."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 10
        mock_runtime.connections.count.return_value = 1

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics/json")

        assert response.status_code == 200
        data = response.json()
        storage = data["data"]["storage"]

        assert "backend" in storage
        assert "rust_core" in storage
        assert storage["backend"] == "RuntimeStorage"
        assert storage["rust_core"] == "v0.50.0"

    def test_get_metrics_json_uptime_measured(self):
        """Test JSON metrics includes uptime measurement."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 10
        mock_runtime.connections.count.return_value = 1

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics/json")

        assert response.status_code == 200
        data = response.json()
        uptime = data["data"]["system"]["uptime_seconds"]

        assert isinstance(uptime, (int, float))
        assert uptime >= 0

    def test_get_metrics_json_error_handling(self):
        """Test JSON metrics handles errors gracefully."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.side_effect = Exception("Storage error")

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/metrics/json")

        # Should return error response
        assert response.status_code == 200
        data = response.json()
        assert data["success"] is False
        assert data["error"]["code"] == "METRICS_ERROR"
        assert data["error"]["message"] == "Failed to generate metrics"


class TestMetricsIntegration:
    """Integration tests for metrics endpoints."""

    def test_both_metrics_endpoints_work(self):
        """Test both Prometheus and JSON endpoints work together."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 50
        mock_runtime.connections.count.return_value = 3

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        with patch('api.routers.metrics.update_system_metrics'):
            with patch('api.routers.metrics.get_metrics_response', return_value=Response(content="# Metrics\n", media_type="text/plain")):
                client = TestClient(app)

                # Get Prometheus metrics
                prom_response = client.get("/api/v1/metrics")
                assert prom_response.status_code == 200

                # Get JSON metrics
                json_response = client.get("/api/v1/metrics/json")
                assert json_response.status_code == 200

                # Both should succeed
                assert prom_response.headers["content-type"] == "text/plain; charset=utf-8"
                assert json_response.json()["success"] is True

    def test_permission_required_for_both_endpoints(self):
        """Test both endpoints require metrics:read permission."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_user = Mock()
        mock_user.scopes = ["some:other:permission"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        client = TestClient(app)

        # Both should fail without permission
        prom_response = client.get("/api/v1/metrics")
        json_response = client.get("/api/v1/metrics/json")

        assert prom_response.status_code == 403
        assert json_response.status_code == 403

    def test_metrics_show_same_counts(self):
        """Test both endpoints show same token/connection counts."""
        app = FastAPI()

        mock_runtime = Mock()
        mock_runtime.tokens.count.return_value = 100
        mock_runtime.connections.count.return_value = 5

        mock_user = Mock()
        mock_user.scopes = ["metrics:read"]

        app.dependency_overrides[metrics.get_runtime] = lambda: mock_runtime
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(metrics_router, prefix="/api/v1")

        with patch('api.routers.metrics.update_system_metrics'):
            with patch('api.routers.metrics.get_metrics_response', return_value=Response(content="", media_type="text/plain")):
                client = TestClient(app)

                # Get JSON metrics to verify counts
                json_response = client.get("/api/v1/metrics/json")

                assert json_response.status_code == 200
                data = json_response.json()

                # Counts should match runtime
                assert data["data"]["tokens"]["active_count"] == 100
                assert data["data"]["connections"]["active_count"] == 5
