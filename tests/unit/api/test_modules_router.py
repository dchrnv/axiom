"""
Unit tests for Modules Router.

Tests module management and configuration endpoints.
"""

import pytest
from unittest.mock import Mock, patch
from fastapi import FastAPI
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

import api.routers.modules as modules
from api.models.modules import ModuleInfo, ModuleStatus, ModuleMetrics
modules_router = modules.router


def create_test_module(module_id="test_module", **kwargs):
    """Helper to create test ModuleInfo object."""
    defaults = {
        "id": module_id,
        "name": f"Test Module {module_id}",
        "description": "Test module description",
        "version": "1.0.0",
        "status": ModuleStatus.ACTIVE,
        "enabled": True,
        "can_disable": True,
        "configurable": True,
        "metrics": ModuleMetrics()
    }
    defaults.update(kwargs)
    return ModuleInfo(**defaults)


class TestListModules:
    """Test GET / endpoint (list modules)."""

    def test_list_modules_success(self):
        """Test successful modules list retrieval."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_modules = [
            create_test_module("module1", enabled=True),
            create_test_module("module2", enabled=False),
        ]

        with patch('api.routers.modules.module_service.list_modules', return_value=mock_modules):
            client = TestClient(app)
            response = client.get("/api/v1/modules")

            assert response.status_code == 200
            data = response.json()
            assert "modules" in data
            assert "total" in data
            assert data["total"] == 2

    def test_list_modules_empty(self):
        """Test modules list when no modules exist."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        with patch('api.routers.modules.module_service.list_modules', return_value=[]):
            client = TestClient(app)
            response = client.get("/api/v1/modules")

            assert response.status_code == 200
            data = response.json()
            assert data["total"] == 0
            assert data["modules"] == []


class TestGetModule:
    """Test GET /{module_id} endpoint."""

    def test_get_module_success(self):
        """Test successful module retrieval."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module")

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            client = TestClient(app)
            response = client.get("/api/v1/modules/test_module")

            assert response.status_code == 200
            data = response.json()
            assert "module" in data

    def test_get_module_not_found(self):
        """Test module not found."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        with patch('api.routers.modules.module_service.get_module', return_value=None):
            client = TestClient(app)
            response = client.get("/api/v1/modules/nonexistent")

            assert response.status_code == 404
            assert "не найден" in response.json()["detail"]


class TestSetModuleEnabled:
    """Test PUT /{module_id}/enabled endpoint."""

    def test_enable_module_success(self):
        """Test successfully enabling a module."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", can_disable=True)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.set_enabled') as mock_set:
                client = TestClient(app)
                response = client.put(
                    "/api/v1/modules/test_module/enabled",
                    json={"enabled": True}
                )

                assert response.status_code == 200
                data = response.json()
                assert data["success"] is True
                assert "включен" in data["message"]
                mock_set.assert_called_once_with("test_module", True)

    def test_disable_module_success(self):
        """Test successfully disabling a module."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", can_disable=True)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.set_enabled') as mock_set:
                client = TestClient(app)
                response = client.put(
                    "/api/v1/modules/test_module/enabled",
                    json={"enabled": False}
                )

                assert response.status_code == 200
                data = response.json()
                assert data["success"] is True
                assert "выключен" in data["message"]
                mock_set.assert_called_once_with("test_module", False)

    def test_disable_core_module_fails(self):
        """Test disabling a core module fails."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("core_module", can_disable=False)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            client = TestClient(app)
            response = client.put(
                "/api/v1/modules/core_module/enabled",
                json={"enabled": False}
            )

            assert response.status_code == 400
            assert "нельзя отключить" in response.json()["detail"]

    def test_enable_core_module_allowed(self):
        """Test enabling a core module is allowed."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("core_module", can_disable=False)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.set_enabled'):
                client = TestClient(app)
                response = client.put(
                    "/api/v1/modules/core_module/enabled",
                    json={"enabled": True}
                )

                # Should succeed even for core modules
                assert response.status_code == 200

    def test_set_enabled_module_not_found(self):
        """Test setting enabled on non-existent module."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        with patch('api.routers.modules.module_service.get_module', return_value=None):
            client = TestClient(app)
            response = client.put(
                "/api/v1/modules/nonexistent/enabled",
                json={"enabled": True}
            )

            assert response.status_code == 404

    def test_set_enabled_service_error(self):
        """Test service error when setting enabled."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", can_disable=True)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.set_enabled', side_effect=Exception("Service error")):
                client = TestClient(app)
                response = client.put(
                    "/api/v1/modules/test_module/enabled",
                    json={"enabled": True}
                )

                assert response.status_code == 500
                assert "Service error" in response.json()["detail"]


class TestGetModuleMetrics:
    """Test GET /{module_id}/metrics endpoint."""

    def test_get_metrics_success(self):
        """Test successful metrics retrieval."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        metrics = ModuleMetrics(operations=100, errors=5, avg_latency_us=15.5)
        mock_module = create_test_module("test_module", metrics=metrics)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            client = TestClient(app)
            response = client.get("/api/v1/modules/test_module/metrics")

            assert response.status_code == 200
            data = response.json()
            assert "metrics" in data
            assert data["metrics"]["operations"] == 100
            assert data["metrics"]["errors"] == 5

    def test_get_metrics_module_not_found(self):
        """Test metrics for non-existent module."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        with patch('api.routers.modules.module_service.get_module', return_value=None):
            client = TestClient(app)
            response = client.get("/api/v1/modules/nonexistent/metrics")

            assert response.status_code == 404

    def test_get_metrics_empty(self):
        """Test metrics when module has no metrics."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module")

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            client = TestClient(app)
            response = client.get("/api/v1/modules/test_module/metrics")

            assert response.status_code == 200
            data = response.json()
            assert "metrics" in data


class TestGetModuleConfig:
    """Test GET /{module_id}/config endpoint."""

    def test_get_config_success(self):
        """Test successful config retrieval."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", configurable=True)
        mock_config = {"setting1": "value1", "setting2": 42}

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.get_config', return_value=mock_config):
                client = TestClient(app)
                response = client.get("/api/v1/modules/test_module/config")

                assert response.status_code == 200
                data = response.json()
                assert "config" in data
                assert data["config"]["setting1"] == "value1"

    def test_get_config_module_not_found(self):
        """Test config for non-existent module."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        with patch('api.routers.modules.module_service.get_module', return_value=None):
            client = TestClient(app)
            response = client.get("/api/v1/modules/nonexistent/config")

            assert response.status_code == 404

    def test_get_config_not_configurable(self):
        """Test config for non-configurable module."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", configurable=False)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            client = TestClient(app)
            response = client.get("/api/v1/modules/test_module/config")

            assert response.status_code == 400
            assert "не поддерживает конфигурацию" in response.json()["detail"]

    def test_get_config_empty(self):
        """Test config when module has no config."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", configurable=True)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.get_config', return_value=None):
                client = TestClient(app)
                response = client.get("/api/v1/modules/test_module/config")

                assert response.status_code == 200
                data = response.json()
                assert data["config"] == {}


class TestSetModuleConfig:
    """Test PUT /{module_id}/config endpoint."""

    def test_set_config_success(self):
        """Test successfully setting module config."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", configurable=True)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.set_config') as mock_set:
                client = TestClient(app)
                response = client.put(
                    "/api/v1/modules/test_module/config",
                    json={"config": {"new_setting": "new_value"}}
                )

                assert response.status_code == 200
                data = response.json()
                assert data["success"] is True
                assert "обновлена" in data["message"]
                mock_set.assert_called_once_with("test_module", {"new_setting": "new_value"})

    def test_set_config_module_not_found(self):
        """Test setting config on non-existent module."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        with patch('api.routers.modules.module_service.get_module', return_value=None):
            client = TestClient(app)
            response = client.put(
                "/api/v1/modules/nonexistent/config",
                json={"config": {}}
            )

            assert response.status_code == 404

    def test_set_config_not_configurable(self):
        """Test setting config on non-configurable module."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", configurable=False)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            client = TestClient(app)
            response = client.put(
                "/api/v1/modules/test_module/config",
                json={"config": {}}
            )

            assert response.status_code == 400
            assert "не поддерживает конфигурацию" in response.json()["detail"]

    def test_set_config_service_error(self):
        """Test service error when setting config."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module("test_module", configurable=True)

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.set_config', side_effect=Exception("Config error")):
                client = TestClient(app)
                response = client.put(
                    "/api/v1/modules/test_module/config",
                    json={"config": {}}
                )

                assert response.status_code == 500
                assert "Config error" in response.json()["detail"]


class TestModulesIntegration:
    """Integration tests for modules endpoints."""

    def test_full_module_workflow(self):
        """Test complete module management workflow."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_module = create_test_module(
            "test_module",
            enabled=True,
            can_disable=True,
            configurable=True
        )

        mock_config = {"setting": "value"}

        with patch('api.routers.modules.module_service.get_module', return_value=mock_module):
            with patch('api.routers.modules.module_service.get_config', return_value=mock_config):
                with patch('api.routers.modules.module_service.set_enabled'):
                    with patch('api.routers.modules.module_service.set_config'):
                        client = TestClient(app)

                        # Get module info
                        info_response = client.get("/api/v1/modules/test_module")
                        assert info_response.status_code == 200

                        # Get metrics
                        metrics_response = client.get("/api/v1/modules/test_module/metrics")
                        assert metrics_response.status_code == 200

                        # Get config
                        config_response = client.get("/api/v1/modules/test_module/config")
                        assert config_response.status_code == 200

                        # Update config
                        update_response = client.put(
                            "/api/v1/modules/test_module/config",
                            json={"config": {"new": "value"}}
                        )
                        assert update_response.status_code == 200

                        # Disable module
                        disable_response = client.put(
                            "/api/v1/modules/test_module/enabled",
                            json={"enabled": False}
                        )
                        assert disable_response.status_code == 200

    def test_list_and_get_consistency(self):
        """Test list and get endpoints return consistent data."""
        app = FastAPI()
        app.include_router(modules_router, prefix="/api/v1/modules")

        mock_modules = [
            create_test_module("module1"),
            create_test_module("module2"),
        ]

        with patch('api.routers.modules.module_service.list_modules', return_value=mock_modules):
            with patch('api.routers.modules.module_service.get_module', return_value=mock_modules[0]):
                client = TestClient(app)

                # List modules
                list_response = client.get("/api/v1/modules")
                assert list_response.status_code == 200

                # Get specific module
                get_response = client.get("/api/v1/modules/module1")
                assert get_response.status_code == 200
