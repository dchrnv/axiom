"""
Comprehensive unit tests for Token Router endpoints.

Tests all token CRUD operations with 8-dimensional coordinate spaces.
Following patterns from test_cdna_router.py and test_grid_router.py.
"""

import pytest
from fastapi import FastAPI
from fastapi.testclient import TestClient
from unittest.mock import Mock, patch
from datetime import datetime
import time
import sys
from pathlib import Path

src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.dependencies import get_token_storage
from api.auth.dependencies import get_current_active_user
import api.routers.tokens as tokens_module

tokens_router = tokens_module.router


# =============================================================================
# Fixtures
# =============================================================================

@pytest.fixture
def mock_token_storage():
    """Mock token storage dependency."""
    storage = Mock()

    # Default mock behaviors
    storage.create = Mock()
    storage.get = Mock()
    storage.list = Mock(return_value=[])
    storage.count = Mock(return_value=0)
    storage.update = Mock()
    storage.delete = Mock()
    storage.clear = Mock(return_value=0)

    return storage


@pytest.fixture
def mock_user_with_all_permissions():
    """Mock user with all token permissions."""
    mock_user = Mock()
    mock_user.scopes = [
        "tokens:read",
        "tokens:write",
        "tokens:delete",
        "config:admin"
    ]
    return mock_user


@pytest.fixture
def mock_user_read_only():
    """Mock user with only read permission."""
    mock_user = Mock()
    mock_user.scopes = ["tokens:read"]
    return mock_user


@pytest.fixture
def mock_user_no_permissions():
    """Mock user with no token permissions."""
    mock_user = Mock()
    mock_user.scopes = []
    return mock_user


@pytest.fixture
def mock_token():
    """Create a mock Token object."""
    token = Mock()
    token.id = 0x01000001  # entity_type=1, domain=0, local_id=1
    token.weight = 0.7
    token.field_radius = 5.0
    token.field_strength = 1.0
    token.timestamp = int(time.time())
    token.has_flag = Mock(side_effect=lambda flag: flag == 0x01)  # FLAG_ACTIVE
    token.get_coordinates = Mock(side_effect=lambda i: (
        (10.5, 20.3, 1.5) if i == 0 else  # L1
        (0.0, 0.0, 0.0)  # Other layers
    ))
    return token


# =============================================================================
# Test Class
# =============================================================================

class TestTokensRouter:
    """Test suite for tokens router endpoints."""

    # -------------------------------------------------------------------------
    # POST /tokens - Create Token
    # -------------------------------------------------------------------------

    def test_create_token_success(self, mock_token_storage, mock_user_with_all_permissions, mock_token):
        """Test successful token creation with L1 coordinates."""
        app = FastAPI()
        mock_token_storage.create.return_value = mock_token

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")

        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.post(
                "/api/v1/tokens",
                json={
                    "entity_type": 1,
                    "domain": 0,
                    "weight": 0.7,
                    "persistent": True,
                    "l1_physical": {"x": 10.5, "y": 20.3, "z": 1.5}
                }
            )

        assert response.status_code == 201
        data = response.json()
        assert data["success"] is True
        assert data["data"]["id"] == 0x01000001
        assert data["data"]["id_hex"] == "0x01000001"
        assert data["data"]["weight"] == 0.7
        assert "L1" in data["data"]["coordinates"]
        mock_token_storage.create.assert_called_once()

    def test_create_token_all_coordinates(self, mock_token_storage, mock_user_with_all_permissions):
        """Test token creation with all 8 coordinate layers."""
        app = FastAPI()
        mock_token = Mock()
        mock_token.id = 0x05000001
        mock_token.weight = 0.8
        mock_token.field_radius = 5.0
        mock_token.field_strength = 1.0
        mock_token.timestamp = int(time.time())
        mock_token.has_flag = Mock(return_value=True)
        mock_token.get_coordinates = Mock(side_effect=lambda i: (
            float(i), float(i+1), float(i+2)
        ))

        mock_token_storage.create.return_value = mock_token

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.post(
                "/api/v1/tokens",
                json={
                    "entity_type": 5,
                    "domain": 0,
                    "weight": 0.8,
                    "l1_physical": {"x": 0.0, "y": 1.0, "z": 2.0},
                    "l2_sensory": {"x": 1.0, "y": 2.0, "z": 3.0},
                    "l3_motor": {"x": 2.0, "y": 3.0, "z": 4.0},
                    "l4_emotional": {"x": 3.0, "y": 4.0, "z": 5.0},
                    "l5_cognitive": {"x": 4.0, "y": 5.0, "z": 6.0},
                    "l6_social": {"x": 5.0, "y": 6.0, "z": 7.0},
                    "l7_temporal": {"x": 6.0, "y": 7.0, "z": 8.0},
                    "l8_abstract": {"x": 7.0, "y": 8.0, "z": 9.0}
                }
            )

        assert response.status_code == 201
        data = response.json()
        assert data["success"] is True
        # Verify all 8 coordinate layers present
        for i in range(1, 9):
            assert f"L{i}" in data["data"]["coordinates"]

    def test_create_token_permission_denied(self, mock_token_storage, mock_user_read_only):
        """Test token creation without write permission."""
        app = FastAPI()
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_read_only
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.post(
                "/api/v1/tokens",
                json={
                    "entity_type": 1,
                    "domain": 0,
                    "weight": 0.5
                }
            )
        assert response.status_code == 403
        assert "tokens:write" in response.json()["detail"]

    def test_create_token_api_disabled(self, mock_token_storage, mock_user_with_all_permissions):
        """Test token creation when API is disabled."""
        app = FastAPI()
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', False):
            client = TestClient(app)
            response = client.post(
                "/api/v1/tokens",
                json={
                    "entity_type": 1,
                    "domain": 0,
                    "weight": 0.5
                }
            )

        assert response.status_code == 503
        assert "not enabled" in response.json()["detail"]

    def test_create_token_storage_error(self, mock_token_storage, mock_user_with_all_permissions):
        """Test token creation with storage error."""
        app = FastAPI()
        mock_token_storage.create.side_effect = Exception("Storage error")

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.post(
                "/api/v1/tokens",
                json={
                    "entity_type": 1,
                    "domain": 0,
                    "weight": 0.5
                }
            )
        assert response.status_code == 500
        assert "Failed to create token" in response.json()["detail"]

    # -------------------------------------------------------------------------
    # GET /tokens - List Tokens
    # -------------------------------------------------------------------------

    def test_list_tokens_success(self, mock_token_storage, mock_user_with_all_permissions, mock_token):
        """Test successful token listing with pagination."""
        app = FastAPI()
        mock_token_storage.list.return_value = [mock_token]
        mock_token_storage.count.return_value = 1

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens?limit=10&offset=0")

        print(f"Status: {response.status_code}, Body: {response.json()}")
        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert len(data["data"]["tokens"]) == 1
        assert data["data"]["total"] == 1
        assert data["data"]["limit"] == 10
        assert data["data"]["offset"] == 0
        mock_token_storage.list.assert_called_once_with(limit=10, offset=0)
        mock_token_storage.count.assert_called_once()

    def test_list_tokens_empty(self, mock_token_storage, mock_user_with_all_permissions):
        """Test listing when no tokens exist."""
        app = FastAPI()
        mock_token_storage.list.return_value = []
        mock_token_storage.count.return_value = 0

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens")
        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert len(data["data"]["tokens"]) == 0
        assert data["data"]["total"] == 0

    def test_list_tokens_pagination(self, mock_token_storage, mock_user_with_all_permissions, mock_token):
        """Test token listing with custom pagination."""
        app = FastAPI()
        mock_token_storage.list.return_value = [mock_token]
        mock_token_storage.count.return_value = 50

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens?limit=25&offset=10")

        assert response.status_code == 200
        data = response.json()
        assert data["data"]["limit"] == 25
        assert data["data"]["offset"] == 10
        assert data["data"]["total"] == 50
        mock_token_storage.list.assert_called_once_with(limit=25, offset=10)

    def test_list_tokens_permission_denied(self, mock_token_storage, mock_user_no_permissions):
        """Test listing without read permission."""
        app = FastAPI()
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_no_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens")
        assert response.status_code == 403
        assert "tokens:read" in response.json()["detail"]

    def test_list_tokens_storage_error(self, mock_token_storage, mock_user_with_all_permissions):
        """Test listing with storage error."""
        app = FastAPI()
        mock_token_storage.list.side_effect = Exception("Storage error")

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens")

        assert response.status_code == 500
        assert "Failed to list tokens" in response.json()["detail"]

    # -------------------------------------------------------------------------
    # GET /tokens/{token_id} - Get Single Token
    # -------------------------------------------------------------------------

    def test_get_token_success(self, mock_token_storage, mock_user_with_all_permissions, mock_token):
        """Test successful token retrieval by ID."""
        app = FastAPI()
        mock_token_storage.get.return_value = mock_token

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens/16777217")  # 0x01000001
        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert data["data"]["id"] == 0x01000001
        assert data["data"]["weight"] == 0.7
        mock_token_storage.get.assert_called_once_with(16777217)

    def test_get_token_not_found(self, mock_token_storage, mock_user_with_all_permissions):
        """Test retrieval of non-existent token."""
        app = FastAPI()
        mock_token_storage.get.return_value = None

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens/999")

        assert response.status_code == 404
        assert "not found" in response.json()["detail"]

    def test_get_token_permission_denied(self, mock_token_storage, mock_user_no_permissions):
        """Test retrieval without read permission."""
        app = FastAPI()
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_no_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens/1")
        assert response.status_code == 403
        assert "tokens:read" in response.json()["detail"]

    def test_get_token_storage_error(self, mock_token_storage, mock_user_with_all_permissions):
        """Test retrieval with storage error."""
        app = FastAPI()
        mock_token_storage.get.side_effect = Exception("Storage error")

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.get("/api/v1/tokens/1")

        assert response.status_code == 500
        assert "Failed to get token" in response.json()["detail"]

    # -------------------------------------------------------------------------
    # PUT /tokens/{token_id} - Update Token
    # -------------------------------------------------------------------------

    def test_update_token_success(self, mock_token_storage, mock_user_with_all_permissions, mock_token):
        """Test successful token update."""
        app = FastAPI()
        mock_token_storage.get.return_value = mock_token

        updated_token = Mock()
        updated_token.id = mock_token.id
        updated_token.weight = 0.9  # Updated weight
        updated_token.field_radius = 2.5  # Updated radius (within valid range)
        updated_token.field_strength = 0.9  # Updated strength (within valid range)
        updated_token.timestamp = mock_token.timestamp
        updated_token.has_flag = mock_token.has_flag
        updated_token.get_coordinates = mock_token.get_coordinates

        mock_token_storage.update.return_value = updated_token

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/tokens/16777217",
                json={
                    "weight": 0.9,
                    "field_radius": 2.5,
                    "field_strength": 0.9
                }
            )
        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert data["data"]["weight"] == 0.9
        assert data["data"]["field_radius"] == 2.5
        assert data["data"]["field_strength"] == 0.9
        mock_token_storage.update.assert_called_once()

    def test_update_token_coordinates(self, mock_token_storage, mock_user_with_all_permissions, mock_token):
        """Test updating token coordinates."""
        app = FastAPI()
        mock_token_storage.get.return_value = mock_token

        updated_token = Mock()
        updated_token.id = mock_token.id
        updated_token.weight = mock_token.weight
        updated_token.field_radius = mock_token.field_radius
        updated_token.field_strength = mock_token.field_strength
        updated_token.timestamp = mock_token.timestamp
        updated_token.has_flag = mock_token.has_flag
        updated_token.get_coordinates = Mock(side_effect=lambda i: (
            (100.0, 200.0, 300.0) if i == 0 else  # Updated L1
            (0.0, 0.0, 0.0)
        ))

        mock_token_storage.update.return_value = updated_token

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/tokens/16777217",
                json={
                    "l1_physical": {"x": 100.0, "y": 200.0, "z": 300.0}
                }
            )

        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True

    def test_update_token_not_found(self, mock_token_storage, mock_user_with_all_permissions):
        """Test updating non-existent token."""
        app = FastAPI()
        mock_token_storage.get.return_value = None

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/tokens/999",
                json={"weight": 0.9}
            )
        assert response.status_code == 404
        assert "not found" in response.json()["detail"]

    def test_update_token_permission_denied(self, mock_token_storage, mock_user_read_only):
        """Test updating without write permission."""
        app = FastAPI()
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_read_only
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/tokens/1",
                json={"weight": 0.9}
            )

        assert response.status_code == 403
        assert "tokens:write" in response.json()["detail"]

    def test_update_token_storage_error(self, mock_token_storage, mock_user_with_all_permissions, mock_token):
        """Test update with storage error."""
        app = FastAPI()
        mock_token_storage.get.return_value = mock_token
        mock_token_storage.update.side_effect = Exception("Storage error")

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.put(
                "/api/v1/tokens/16777217",
                json={"weight": 0.9}
            )
        assert response.status_code == 500
        assert "Failed to update token" in response.json()["detail"]

    # -------------------------------------------------------------------------
    # DELETE /tokens/{token_id} - Delete Token
    # -------------------------------------------------------------------------

    def test_delete_token_success(self, mock_token_storage, mock_user_with_all_permissions):
        """Test successful token deletion."""
        app = FastAPI()
        mock_token_storage.delete.return_value = True

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/tokens/16777217")

        assert response.status_code == 204
        mock_token_storage.delete.assert_called_once_with(16777217)

    def test_delete_token_not_found(self, mock_token_storage, mock_user_with_all_permissions):
        """Test deleting non-existent token."""
        app = FastAPI()
        mock_token_storage.delete.return_value = False

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/tokens/999")
        assert response.status_code == 404
        assert "not found" in response.json()["detail"]

    def test_delete_token_permission_denied(self, mock_token_storage, mock_user_read_only):
        """Test deleting without delete permission."""
        app = FastAPI()
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_read_only
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/tokens/1")

        assert response.status_code == 403
        assert "tokens:delete" in response.json()["detail"]

    def test_delete_token_storage_error(self, mock_token_storage, mock_user_with_all_permissions):
        """Test deletion with storage error."""
        app = FastAPI()
        mock_token_storage.delete.side_effect = Exception("Storage error")

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/tokens/1")
        assert response.status_code == 500
        assert "Failed to delete token" in response.json()["detail"]

    # -------------------------------------------------------------------------
    # POST /tokens/examples/create - Create Example Tokens
    # -------------------------------------------------------------------------

    def test_create_examples_success(self, mock_token_storage, mock_user_with_all_permissions):
        """Test creating example tokens."""
        app = FastAPI()
        # Mock two example tokens
        token1 = Mock()
        token1.id = 0x01000001
        token1.weight = 0.7
        token1.field_radius = 5.0
        token1.field_strength = 1.0
        token1.timestamp = int(time.time())
        token1.has_flag = Mock(return_value=True)
        token1.get_coordinates = Mock(side_effect=lambda i: (
            (10.5, 20.3, 1.5) if i == 0 else (0.0, 0.0, 0.0)
        ))

        token2 = Mock()
        token2.id = 0x05000001
        token2.weight = 0.8
        token2.field_radius = 5.0
        token2.field_strength = 1.0
        token2.timestamp = int(time.time())
        token2.has_flag = Mock(return_value=False)
        token2.get_coordinates = Mock(side_effect=lambda i: (
            (0.8, 0.5, 0.3) if i == 3 else (0.0, 0.0, 0.0)  # L4 emotional
        ))

        mock_token_storage.create.side_effect = [token1, token2]

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/tokens/examples/create")

        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert data["data"]["count"] == 2
        assert len(data["data"]["examples"]) == 2
        assert mock_token_storage.create.call_count == 2

    def test_create_examples_permission_denied(self, mock_token_storage, mock_user_read_only):
        """Test creating examples without write permission."""
        app = FastAPI()
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_read_only
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/tokens/examples/create")
        assert response.status_code == 403
        assert "tokens:write" in response.json()["detail"]

    def test_create_examples_storage_error(self, mock_token_storage, mock_user_with_all_permissions):
        """Test creating examples with storage error."""
        app = FastAPI()
        mock_token_storage.create.side_effect = Exception("Storage error")

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/tokens/examples/create")

        assert response.status_code == 500
        assert "Failed to create examples" in response.json()["detail"]

    # -------------------------------------------------------------------------
    # DELETE /tokens/admin/clear - Clear All Tokens
    # -------------------------------------------------------------------------

    def test_clear_all_tokens_success(self, mock_token_storage, mock_user_with_all_permissions):
        """Test clearing all tokens (admin)."""
        app = FastAPI()
        mock_token_storage.clear.return_value = 42

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/tokens/admin/clear")
        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert data["data"]["cleared"] == 42
        assert "42 tokens" in data["data"]["message"]
        mock_token_storage.clear.assert_called_once()

    def test_clear_all_tokens_empty(self, mock_token_storage, mock_user_with_all_permissions):
        """Test clearing when no tokens exist."""
        app = FastAPI()
        mock_token_storage.clear.return_value = 0

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/tokens/admin/clear")

        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert data["data"]["cleared"] == 0

    def test_clear_all_tokens_permission_denied(self, mock_token_storage, mock_user_read_only):
        """Test clearing without admin permission."""
        app = FastAPI()
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_read_only
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/tokens/admin/clear")
        assert response.status_code == 403
        assert "config:admin" in response.json()["detail"]

    def test_clear_all_tokens_storage_error(self, mock_token_storage, mock_user_with_all_permissions):
        """Test clearing with storage error."""
        app = FastAPI()
        mock_token_storage.clear.side_effect = Exception("Storage error")

        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user_with_all_permissions
        app.include_router(tokens_router, prefix="/api/v1")


        with patch('src.api.routers.tokens.settings.ENABLE_NEW_TOKEN_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/tokens/admin/clear")

        assert response.status_code == 500
        assert "Failed to clear tokens" in response.json()["detail"]
