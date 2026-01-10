"""
Unit tests for API Keys Router.

Tests CRUD operations for API key management.
"""

import pytest
from unittest.mock import Mock, patch
from fastapi import FastAPI
from fastapi.testclient import TestClient
from datetime import datetime, timedelta

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

import api.routers.api_keys as api_keys
from api.auth.dependencies import get_current_active_user
api_keys_router = api_keys.router


class TestCreateAPIKey:
    """Test POST /api-keys endpoint."""

    def test_create_key_success(self):
        """Test successful API key creation."""
        app = FastAPI()

        # Mock user with admin permission
        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(api_keys_router, prefix="/api/v1")

        # Mock API key storage
        mock_storage = Mock()
        mock_key = Mock()
        mock_key.key_id = "key_123"
        mock_key.key_prefix = "ng_"
        mock_key.name = "test-key"
        mock_key.scopes = ["read"]
        mock_key.rate_limit = 100
        mock_key.created_at = datetime.utcnow()
        mock_key.expires_at = datetime.utcnow() + timedelta(days=30)

        mock_storage.generate_key.return_value = ("ng_fullkeyhere", mock_key)

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.post(
                "/api/v1/api-keys",
                json={
                    "name": "test-key",
                    "scopes": ["read"],
                    "rate_limit": 100,
                    "expires_in_days": 30
                }
            )

            assert response.status_code == 201
            data = response.json()
            assert data["success"] is True
            assert "api_key" in data["data"]
            assert data["data"]["api_key"] == "ng_fullkeyhere"
            assert data["data"]["key_id"] == "key_123"
            assert "warning" in data["data"]

    def test_create_key_without_permission(self):
        """Test that key creation requires config:admin permission."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        # Mock user without admin permission
        mock_user = Mock()
        mock_user.user_id = "viewer"
        mock_user.scopes = ["read"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        client = TestClient(app)
        response = client.post(
            "/api/v1/api-keys",
            json={"name": "test-key", "scopes": ["read"]}
        )

        assert response.status_code == 403
        assert "permission denied" in response.json()["detail"].lower()

    def test_create_key_without_auth(self):
        """Test that key creation requires authentication."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/api-keys",
            json={"name": "test-key", "scopes": ["read"]}
        )

        assert response.status_code in [401, 403]

    def test_create_key_storage_error(self):
        """Test key creation with storage error."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_storage.generate_key.side_effect = Exception("Storage error")

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.post(
                "/api/v1/api-keys",
                json={"name": "test-key", "scopes": ["read"]}
            )

            assert response.status_code == 500
            assert "failed to create" in response.json()["detail"].lower()

    def test_create_key_returns_full_key_once(self):
        """Test that full key is returned only during creation."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_key = Mock()
        mock_key.key_id = "key_456"
        mock_key.key_prefix = "ng_"
        mock_key.name = "secret-key"
        mock_key.scopes = ["write"]
        mock_key.rate_limit = 50
        mock_key.created_at = datetime.utcnow()
        mock_key.expires_at = None

        mock_storage.generate_key.return_value = ("ng_secretfullkey", mock_key)

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.post(
                "/api/v1/api-keys",
                json={"name": "secret-key", "scopes": ["write"], "rate_limit": 50}
            )

            data = response.json()
            assert "api_key" in data["data"]
            assert data["data"]["api_key"] == "ng_secretfullkey"
            assert "warning" in data["data"]
            assert "not be shown again" in data["data"]["warning"].lower()


class TestListAPIKeys:
    """Test GET /api-keys endpoint."""

    def test_list_keys_success(self):
        """Test successful API keys listing."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_key1 = Mock()
        mock_key1.key_id = "key_1"
        mock_key1.key_prefix = "ng_"
        mock_key1.name = "key-1"
        mock_key1.scopes = ["read"]
        mock_key1.rate_limit = 100
        mock_key1.created_at = datetime.utcnow()
        mock_key1.expires_at = None
        mock_key1.last_used_at = None
        mock_key1.disabled = False

        mock_key2 = Mock()
        mock_key2.key_id = "key_2"
        mock_key2.key_prefix = "ng_"
        mock_key2.name = "key-2"
        mock_key2.scopes = ["write"]
        mock_key2.rate_limit = 50
        mock_key2.created_at = datetime.utcnow()
        mock_key2.expires_at = None
        mock_key2.last_used_at = datetime.utcnow()
        mock_key2.disabled = False

        mock_storage.list_keys.return_value = [mock_key1, mock_key2]

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.get("/api/v1/api-keys")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert "keys" in data["data"]
            assert "count" in data["data"]
            assert data["data"]["count"] == 2
            assert len(data["data"]["keys"]) == 2

    def test_list_keys_empty(self):
        """Test listing with no keys."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_storage.list_keys.return_value = []

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.get("/api/v1/api-keys")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["count"] == 0
            assert data["data"]["keys"] == []

    def test_list_keys_without_permission(self):
        """Test that listing requires config:admin permission."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "viewer"
        mock_user.scopes = ["read"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        client = TestClient(app)
        response = client.get("/api/v1/api-keys")

        assert response.status_code == 403

    def test_list_keys_no_full_keys(self):
        """Test that listing doesn't include full key values."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_key = Mock()
        mock_key.key_id = "key_1"
        mock_key.key_prefix = "ng_"
        mock_key.name = "key-1"
        mock_key.scopes = ["read"]
        mock_key.rate_limit = 100
        mock_key.created_at = datetime.utcnow()
        mock_key.expires_at = None
        mock_key.last_used_at = None
        mock_key.disabled = False

        mock_storage.list_keys.return_value = [mock_key]

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.get("/api/v1/api-keys")

            data = response.json()
            # Verify no full key in response
            for key in data["data"]["keys"]:
                assert "api_key" not in key
                assert "key_id" in key
                assert "key_prefix" in key


class TestGetAPIKey:
    """Test GET /api-keys/{key_id} endpoint."""

    def test_get_key_success(self):
        """Test successful API key retrieval."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_key = Mock()
        mock_key.key_id = "key_123"
        mock_key.key_prefix = "ng_"
        mock_key.name = "test-key"
        mock_key.scopes = ["read", "write"]
        mock_key.rate_limit = 100
        mock_key.created_at = datetime.utcnow()
        mock_key.expires_at = None
        mock_key.last_used_at = None
        mock_key.disabled = False

        mock_storage.get_key.return_value = mock_key

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.get("/api/v1/api-keys/key_123")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert data["data"]["key_id"] == "key_123"
            assert data["data"]["name"] == "test-key"

    def test_get_key_not_found(self):
        """Test getting non-existent key."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_storage.get_key.return_value = None

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.get("/api/v1/api-keys/nonexistent")

            assert response.status_code == 404
            assert "not found" in response.json()["detail"].lower()

    def test_get_key_without_permission(self):
        """Test that getting key requires permission."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "viewer"
        mock_user.scopes = ["read"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        client = TestClient(app)
        response = client.get("/api/v1/api-keys/key_123")

        assert response.status_code == 403


class TestRevokeAPIKey:
    """Test POST /api-keys/{key_id}/revoke endpoint."""

    def test_revoke_key_success(self):
        """Test successful API key revocation."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_storage.revoke_key.return_value = True

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.post("/api/v1/api-keys/key_123/revoke")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert data["data"]["key_id"] == "key_123"
            assert data["data"]["status"] == "revoked"

    def test_revoke_key_not_found(self):
        """Test revoking non-existent key."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_storage.revoke_key.return_value = False

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.post("/api/v1/api-keys/nonexistent/revoke")

            assert response.status_code == 404

    def test_revoke_key_without_permission(self):
        """Test that revoking requires permission."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "viewer"
        mock_user.scopes = ["read"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        client = TestClient(app)
        response = client.post("/api/v1/api-keys/key_123/revoke")

        assert response.status_code == 403


class TestDeleteAPIKey:
    """Test DELETE /api-keys/{key_id} endpoint."""

    def test_delete_key_success(self):
        """Test successful API key deletion."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_storage.delete_key.return_value = True

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.delete("/api/v1/api-keys/key_123")

            assert response.status_code == 204

    def test_delete_key_not_found(self):
        """Test deleting non-existent key."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()
        mock_storage.delete_key.return_value = False

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)
            response = client.delete("/api/v1/api-keys/nonexistent")

            assert response.status_code == 404

    def test_delete_key_without_permission(self):
        """Test that deleting requires permission."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "viewer"
        mock_user.scopes = ["read"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        client = TestClient(app)
        response = client.delete("/api/v1/api-keys/key_123")

        assert response.status_code == 403


class TestAPIKeysIntegration:
    """Test API keys endpoints working together."""

    def test_full_key_lifecycle(self):
        """Test create, list, get, revoke, delete flow."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        mock_user = Mock()
        mock_user.user_id = "admin"
        mock_user.scopes = ["config:admin"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        mock_storage = Mock()

        # Setup mock key
        mock_key = Mock()
        mock_key.key_id = "key_test"
        mock_key.key_prefix = "ng_"
        mock_key.name = "test-key"
        mock_key.scopes = ["read"]
        mock_key.rate_limit = 100
        mock_key.created_at = datetime.utcnow()
        mock_key.expires_at = None
        mock_key.last_used_at = None
        mock_key.disabled = False

        # Mock storage methods
        mock_storage.generate_key.return_value = ("ng_fullkey", mock_key)
        mock_storage.list_keys.return_value = [mock_key]
        mock_storage.get_key.return_value = mock_key
        mock_storage.revoke_key.return_value = True
        mock_storage.delete_key.return_value = True

        with patch('api.routers.api_keys.get_api_key_storage', return_value=mock_storage):
            client = TestClient(app)

            # 1. Create key
            create_resp = client.post(
                "/api/v1/api-keys",
                json={"name": "test-key", "scopes": ["read"], "rate_limit": 100}
            )
            assert create_resp.status_code == 201
            assert "api_key" in create_resp.json()["data"]

            # 2. List keys
            list_resp = client.get("/api/v1/api-keys")
            assert list_resp.status_code == 200
            assert list_resp.json()["data"]["count"] == 1

            # 3. Get specific key
            get_resp = client.get("/api/v1/api-keys/key_test")
            assert get_resp.status_code == 200
            assert get_resp.json()["data"]["key_id"] == "key_test"

            # 4. Revoke key
            revoke_resp = client.post("/api/v1/api-keys/key_test/revoke")
            assert revoke_resp.status_code == 200
            assert revoke_resp.json()["data"]["status"] == "revoked"

            # 5. Delete key
            delete_resp = client.delete("/api/v1/api-keys/key_test")
            assert delete_resp.status_code == 204

    def test_permission_required_for_all_operations(self):
        """Test that all operations require config:admin permission."""
        app = FastAPI()
        app.include_router(api_keys_router, prefix="/api/v1")

        # User without permission
        mock_user = Mock()
        mock_user.user_id = "viewer"
        mock_user.scopes = ["read"]
        app.dependency_overrides[api_keys.get_current_active_user] = lambda: mock_user

        client = TestClient(app)

        # All operations should fail with 403
        operations = [
            ("post", "/api/v1/api-keys", {"name": "test", "scopes": ["read"]}),
            ("get", "/api/v1/api-keys", None),
            ("get", "/api/v1/api-keys/key_123", None),
            ("post", "/api/v1/api-keys/key_123/revoke", None),
            ("delete", "/api/v1/api-keys/key_123", None),
        ]

        for method, path, json_data in operations:
            if method == "post":
                response = client.post(path, json=json_data) if json_data else client.post(path)
            elif method == "get":
                response = client.get(path)
            elif method == "delete":
                response = client.delete(path)

            assert response.status_code == 403, f"{method.upper()} {path} should require permission"
