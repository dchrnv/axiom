"""
Unit tests for CDNA Router.

Tests Cognitive DNA configuration, profiles, quarantine mode, and validation.
"""

import pytest
from unittest.mock import Mock, patch
from fastapi import FastAPI
from fastapi.testclient import TestClient
from datetime import datetime

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.auth.dependencies import get_current_active_user
import api.routers.cdna as cdna_module
from api.dependencies import get_cdna_storage

cdna_router = cdna_module.router


class TestGetCDNAStatus:
    """Test GET /cdna/status endpoint."""

    def test_get_status_success(self):
        """Test successful CDNA status retrieval."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_config.return_value = {
            "version": "2.1.0",
            "profile": "explorer",
            "dimension_scales": [1.0] * 8,
            "timestamp": "2024-01-01T00:00:00Z"
        }
        mock_storage.get_quarantine_status.return_value = {
            "enabled": False,
            "started_at": None
        }
        mock_storage.get_history.return_value = [
            {"action": "config_update", "timestamp": "2024-01-01T00:00:00Z"}
        ]

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/status")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert "data" in data
            assert data["data"]["cdna"]["profile"] == "explorer"
            assert data["data"]["history_count"] == 1

    def test_get_status_without_permission(self):
        """Test status fails without permission."""
        app = FastAPI()

        mock_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = []

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/status")

            assert response.status_code == 403
            assert "cdna:read" in response.json()["detail"]

    def test_get_status_api_disabled(self):
        """Test status fails when API is disabled."""
        app = FastAPI()

        mock_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', False):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/status")

            assert response.status_code == 503
            assert "not enabled" in response.json()["detail"]

    def test_get_status_storage_error(self):
        """Test status handles storage errors."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_config.side_effect = Exception("Storage error")

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/status")

            assert response.status_code == 500
            assert "Storage error" in response.json()["detail"]


class TestUpdateCDNAConfig:
    """Test PUT /cdna/config endpoint."""

    def test_update_config_with_profile(self):
        """Test updating config by switching profile."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_config.return_value = {"version": "2.1.0", "profile": "explorer"}
        mock_storage.get_profile.return_value = {
            "name": "analyzer",
            "scales": [0.8, 1.2, 1.0, 0.9, 1.5, 1.0, 1.0, 1.1]
        }
        mock_storage.update_config.return_value = True
        mock_storage.add_history.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/cdna/config",
                json={"profile": "analyzer", "should_validate": False}
            )

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            mock_storage.update_config.assert_called_once()
            mock_storage.add_history.assert_called_once()

    def test_update_config_with_scales(self):
        """Test updating config with custom scales."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_config.return_value = {"version": "2.1.0"}
        mock_storage.update_config.return_value = True
        mock_storage.add_history.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/cdna/config",
                json={
                    "dimension_scales": [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                    "should_validate": False
                }
            )

            assert response.status_code == 200
            assert mock_storage.update_config.called

    def test_update_config_with_validation(self):
        """Test config update with validation enabled."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_config.return_value = {"version": "2.1.0"}
        mock_storage.validate_scales.return_value = (True, [], [])
        mock_storage.update_config.return_value = True
        mock_storage.add_history.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/cdna/config",
                json={
                    "dimension_scales": [1.0] * 8,
                    "should_validate": True
                }
            )

            assert response.status_code == 200
            mock_storage.validate_scales.assert_called_once()

    def test_update_config_validation_fails(self):
        """Test config update fails on validation error."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_config.return_value = {"version": "2.1.0"}
        mock_storage.validate_scales.return_value = (
            False,
            ["Warning message"],
            ["Error: Invalid scale"]
        )

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/cdna/config",
                json={
                    "dimension_scales": [999.0] * 8,
                    "should_validate": True
                }
            )

            assert response.status_code == 400
            assert "validation failed" in response.json()["detail"]["message"]

    def test_update_config_without_permission(self):
        """Test update fails without permission."""
        app = FastAPI()

        mock_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]  # Has read but not write

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/cdna/config",
                json={"profile": "analyzer"}
            )

            assert response.status_code == 403
            assert "cdna:write" in response.json()["detail"]

    def test_update_config_profile_not_found(self):
        """Test update fails when profile doesn't exist."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_config.return_value = {"version": "2.1.0"}
        mock_storage.get_profile.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/cdna/config",
                json={"profile": "nonexistent", "should_validate": True}
            )

            assert response.status_code == 404
            assert "not found" in response.json()["detail"]


class TestListProfiles:
    """Test GET /cdna/profiles endpoint."""

    def test_list_profiles_success(self):
        """Test successful profiles listing."""
        app = FastAPI()

        mock_storage = Mock()
        # list_profiles returns a dict, not a list
        mock_storage.list_profiles.return_value = {
            "explorer": {"name": "explorer", "scales": [1.0] * 8, "description": "Explorer profile"},
            "analyzer": {"name": "analyzer", "scales": [0.8, 1.2, 1.0, 0.9, 1.5, 1.0, 1.0, 1.1], "description": "Analyzer profile"},
        }
        mock_storage.get_config.return_value = {"profile": "explorer"}

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/profiles")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert len(data["data"]["profiles"]) == 2
            assert "explorer" in data["data"]["profiles"]

    def test_list_profiles_without_permission(self):
        """Test listing fails without permission."""
        app = FastAPI()

        mock_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = []

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/profiles")

            assert response.status_code == 403
            assert "cdna:read" in response.json()["detail"]


class TestGetProfile:
    """Test GET /cdna/profiles/{profile_id} endpoint."""

    def test_get_profile_success(self):
        """Test successful profile retrieval."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_profile.return_value = {
            "name": "explorer",
            "scales": [1.0] * 8,
            "description": "Balanced exploration profile",
            "plasticity": 0.5,
            "evolution_rate": 0.5
        }

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/profiles/explorer")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["name"] == "explorer"
            assert len(data["data"]["scales"]) == 8

    def test_get_profile_not_found(self):
        """Test getting non-existent profile."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_profile.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/profiles/nonexistent")

            assert response.status_code == 404
            assert "not found" in response.json()["detail"]


class TestSwitchProfile:
    """Test POST /cdna/profiles/{profile_id}/switch endpoint."""

    def test_switch_profile_success(self):
        """Test successful profile switch."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_profile.return_value = {
            "name": "analyzer",
            "scales": [0.8, 1.2, 1.0, 0.9, 1.5, 1.0, 1.0, 1.1]
        }
        mock_storage.get_config.return_value = {"version": "2.1.0", "profile": "explorer"}
        mock_storage.switch_profile.return_value = True
        mock_storage.add_history.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/profiles/analyzer/switch")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            mock_storage.switch_profile.assert_called_once_with("analyzer")

    def test_switch_profile_not_found(self):
        """Test switching to non-existent profile."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_profile.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/profiles/nonexistent/switch")

            assert response.status_code == 404


class TestValidateConfig:
    """Test POST /cdna/validate endpoint."""

    def test_validate_config_success(self):
        """Test successful configuration validation."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.validate_scales.return_value = (True, ["Info message"], [])

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post(
                "/api/v1/cdna/validate",
                json={"scales": [1.0] * 8}  # Field is "scales", not "dimension_scales"
            )

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["valid"] is True
            assert len(data["data"]["warnings"]) == 1

    def test_validate_config_invalid(self):
        """Test validation with invalid configuration."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.validate_scales.return_value = (
            False,
            [],
            ["Scale out of range"]
        )

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post(
                "/api/v1/cdna/validate",
                json={"scales": [999.0] * 8}  # Field is "scales", not "dimension_scales"
            )

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["valid"] is False
            assert len(data["data"]["errors"]) == 1


class TestQuarantineStatus:
    """Test GET /cdna/quarantine/status endpoint."""

    def test_get_quarantine_status(self):
        """Test getting quarantine status."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_quarantine_status.return_value = {
            "active": True,
            "time_left": 150,
            "metrics": {}
        }

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/quarantine/status")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["active"] is True
            assert data["data"]["time_left"] == 150


class TestQuarantineStart:
    """Test POST /cdna/quarantine/start endpoint."""

    def test_start_quarantine_success(self):
        """Test successfully starting quarantine mode."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_quarantine_status.return_value = {"active": False}
        mock_storage.start_quarantine.return_value = True
        mock_storage.add_history.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/quarantine/start")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            mock_storage.start_quarantine.assert_called_once()

    def test_start_quarantine_already_enabled(self):
        """Test starting quarantine when already enabled."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_quarantine_status.return_value = {"active": True}

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/quarantine/start")

            assert response.status_code == 400
            assert "already active" in response.json()["detail"]


class TestQuarantineStop:
    """Test POST /cdna/quarantine/stop endpoint."""

    def test_stop_quarantine_success(self):
        """Test successfully stopping quarantine mode."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_quarantine_status.return_value = {"active": True}
        mock_storage.stop_quarantine.return_value = True
        mock_storage.add_history.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/quarantine/stop")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            mock_storage.stop_quarantine.assert_called_once()

    def test_stop_quarantine_not_enabled(self):
        """Test stopping quarantine when not enabled."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_quarantine_status.return_value = {"active": False}

        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/quarantine/stop")

            assert response.status_code == 400
            assert "not active" in response.json()["detail"]


class TestGetHistory:
    """Test GET /cdna/history endpoint."""

    def test_get_history_success(self):
        """Test successful history retrieval."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_history.return_value = [
            {"action": "config_update", "timestamp": "2024-01-01T00:00:00Z", "changes": {}},
            {"action": "profile_switch", "timestamp": "2024-01-02T00:00:00Z", "changes": {}},
        ]

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/history")

            assert response.status_code == 200
            data = response.json()
            assert len(data["data"]["history"]) == 2
            assert data["data"]["total"] == 2
            # get_history is called twice - once for entries, once for count
            assert mock_storage.get_history.call_count == 2

    def test_get_history_with_limit(self):
        """Test history retrieval with custom limit."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_history.return_value = [
            {"action": "config_update", "timestamp": "2024-01-01T00:00:00Z"}
        ]

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/cdna/history?limit=10")

            assert response.status_code == 200
            # get_history is called twice - once with limit=10, once with limit=1000 for total count
            assert mock_storage.get_history.call_count == 2
            # First call should be with our custom limit
            first_call = mock_storage.get_history.call_args_list[0]
            assert first_call[1]["limit"] == 10


class TestExportConfig:
    """Test POST /cdna/export endpoint."""

    def test_export_config_success(self):
        """Test successful config export."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.get_config.return_value = {
            "version": "2.1.0",
            "profile": "explorer",
            "dimension_scales": [1.0] * 8
        }
        mock_storage.get_history.return_value = []

        mock_user = Mock()
        mock_user.scopes = ["cdna:read"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/export")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert "data" in data["data"]
            assert "cdna" in data["data"]["data"]
            assert "history" in data["data"]["data"]
            assert "filename" in data["data"]


class TestResetConfig:
    """Test POST /cdna/reset endpoint."""

    def test_reset_config_success(self):
        """Test successful config reset."""
        app = FastAPI()

        mock_storage = Mock()
        mock_storage.update_config.return_value = True
        mock_storage.get_config.return_value = {
            "version": "2.1.0",
            "profile": "explorer",
            "dimension_scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0]
        }
        mock_storage.add_history.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["config:admin"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/reset")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert data["data"]["cdna"]["profile"] == "explorer"
            mock_storage.update_config.assert_called_once()
            mock_storage.add_history.assert_called_once()

    def test_reset_config_without_admin(self):
        """Test reset fails without admin permission."""
        app = FastAPI()

        mock_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = ["cdna:write"]  # Has write but not admin

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/cdna/reset")

            assert response.status_code == 403
            assert "config:admin" in response.json()["detail"]


class TestCDNAIntegration:
    """Integration tests for CDNA endpoints."""

    def test_full_cdna_workflow(self):
        """Test complete CDNA configuration workflow."""
        app = FastAPI()

        mock_storage = Mock()

        # Setup all mock responses
        mock_storage.get_config.return_value = {
            "version": "2.1.0",
            "profile": "explorer",
            "dimension_scales": [1.0] * 8,
            "timestamp": "2024-01-01T00:00:00Z"
        }
        mock_storage.list_profiles.return_value = {
            "explorer": {"name": "explorer", "scales": [1.0] * 8}
        }
        mock_storage.get_profile.return_value = {
            "name": "analyzer",
            "scales": [0.8, 1.2, 1.0, 0.9, 1.5, 1.0, 1.0, 1.1]
        }
        mock_storage.switch_profile.return_value = True
        mock_storage.validate_scales.return_value = (True, [], [])
        mock_storage.add_history.return_value = None
        mock_storage.get_history.return_value = []
        mock_storage.get_quarantine_status.return_value = {"active": False}

        mock_user = Mock()
        mock_user.scopes = ["cdna:read", "cdna:write"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', True):
            client = TestClient(app)

            # Get status
            status_response = client.get("/api/v1/cdna/status")
            assert status_response.status_code == 200

            # List profiles
            profiles_response = client.get("/api/v1/cdna/profiles")
            assert profiles_response.status_code == 200

            # Switch profile
            switch_response = client.post("/api/v1/cdna/profiles/analyzer/switch")
            assert switch_response.status_code == 200

            # Validate config
            validate_response = client.post(
                "/api/v1/cdna/validate",
                json={"scales": [1.0] * 8}
            )
            assert validate_response.status_code == 200

            # Get history
            history_response = client.get("/api/v1/cdna/history")
            assert history_response.status_code == 200

    def test_api_disabled_all_endpoints(self):
        """Test all endpoints fail when API is disabled."""
        app = FastAPI()

        mock_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = ["cdna:read", "cdna:write", "config:admin"]

        app.dependency_overrides[get_cdna_storage] = lambda: mock_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(cdna_router, prefix="/api/v1")

        with patch('api.routers.cdna.settings.ENABLE_NEW_CDNA_API', False):
            client = TestClient(app)

            # All endpoints should return 503
            endpoints = [
                ("GET", "/api/v1/cdna/status"),
                ("GET", "/api/v1/cdna/profiles"),
                ("GET", "/api/v1/cdna/profiles/explorer"),
                ("GET", "/api/v1/cdna/quarantine/status"),
                ("GET", "/api/v1/cdna/history"),
            ]

            for method, url in endpoints:
                response = client.get(url) if method == "GET" else client.post(url)
                assert response.status_code == 503, f"{method} {url} should return 503"
                assert "not enabled" in response.json()["detail"]
