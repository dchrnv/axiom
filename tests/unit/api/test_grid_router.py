"""
Unit tests for Grid Router.

Tests spatial indexing, neighbor search, range queries, and grid management.
"""

import pytest
from unittest.mock import Mock, patch, MagicMock
from fastapi import FastAPI
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.auth.dependencies import get_current_active_user
import api.routers.grid as grid_module
from api.dependencies import get_token_storage, get_grid_storage

grid_router = grid_module.router


class TestGridStatus:
    """Test GET /grid/status endpoint."""

    def test_grid_status_available(self):
        """Test grid status when available."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.list_grids.return_value = [1, 2, 3]  # 3 grids

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/status")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert data["data"]["available"] is True
            assert data["data"]["grids_count"] == 3
            assert "ready" in data["data"]["message"]

    def test_grid_status_unavailable(self):
        """Test grid status when not available."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.check_grid_available', return_value=False):
            client = TestClient(app)
            response = client.get("/api/v1/grid/status")

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert data["data"]["available"] is False
            assert data["data"]["grids_count"] == 0
            assert "not available" in data["data"]["message"]

    def test_grid_status_without_permission(self):
        """Test status fails without permission."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = []

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/status")

            assert response.status_code == 403
            assert "grid:read" in response.json()["detail"]

    def test_grid_status_disabled(self):
        """Test status when grid is disabled."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', False):
            client = TestClient(app)
            response = client.get("/api/v1/grid/status")

            assert response.status_code == 503
            assert "disabled" in response.json()["detail"]


class TestCreateGrid:
    """Test POST /grid/create endpoint."""

    def test_create_grid_with_default_config(self):
        """Test creating grid with default configuration."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.list_grids.return_value = []
        mock_grid_storage.create_grid.return_value = 1

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.settings.GRID_MAX_INSTANCES', 10), \
             patch('api.routers.grid.settings.GRID_DEFAULT_BUCKET_SIZE', 10.0), \
             patch('api.routers.grid.settings.GRID_DEFAULT_DENSITY_THRESHOLD', 0.5), \
             patch('api.routers.grid.settings.GRID_DEFAULT_MIN_FIELD_NODES', 3), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.post("/api/v1/grid/create")

            assert response.status_code == 201
            data = response.json()
            assert data["success"] is True
            assert data["data"]["grid_id"] == 1
            assert data["data"]["status"] == "created"
            assert data["data"]["config"]["bucket_size"] == 10.0
            mock_grid_storage.create_grid.assert_called_once()

    def test_create_grid_with_custom_config(self):
        """Test creating grid with custom configuration."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.list_grids.return_value = []
        mock_grid_storage.create_grid.return_value = 2

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.settings.GRID_MAX_INSTANCES', 10), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.post(
                "/api/v1/grid/create",
                json={
                    "bucket_size": 15.0,
                    "density_threshold": 0.7,
                    "min_field_nodes": 5
                }
            )

            assert response.status_code == 201
            data = response.json()
            assert data["data"]["config"]["bucket_size"] == 15.0
            assert data["data"]["config"]["density_threshold"] == 0.7
            assert data["data"]["config"]["min_field_nodes"] == 5

    def test_create_grid_max_instances_reached(self):
        """Test creating grid when max instances reached."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.list_grids.return_value = [1, 2, 3, 4, 5]  # 5 grids

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.settings.GRID_MAX_INSTANCES', 5), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.post("/api/v1/grid/create")

            assert response.status_code == 400
            assert "Maximum grid instances" in response.json()["detail"]

    def test_create_grid_not_available(self):
        """Test creating grid when not available."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.list_grids.return_value = []

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=False):
            client = TestClient(app)
            response = client.post("/api/v1/grid/create")

            assert response.status_code == 503
            assert "not available" in response.json()["detail"]

    def test_create_grid_without_permission(self):
        """Test grid creation without permission."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = ["grid:read"]  # Has read but not write

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True):
            client = TestClient(app)
            response = client.post("/api/v1/grid/create")

            assert response.status_code == 403
            assert "grid:write" in response.json()["detail"]


class TestGetGridInfo:
    """Test GET /grid/{grid_id} endpoint."""

    def test_get_grid_info_success(self):
        """Test successful grid info retrieval."""
        app = FastAPI()

        mock_grid = Mock()
        mock_grid.__len__ = Mock(return_value=5)  # 5 tokens

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.settings.GRID_DEFAULT_BUCKET_SIZE', 10.0), \
             patch('api.routers.grid.settings.GRID_DEFAULT_DENSITY_THRESHOLD', 0.5), \
             patch('api.routers.grid.settings.GRID_DEFAULT_MIN_FIELD_NODES', 3), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/1")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["grid_id"] == 1
            assert data["data"]["token_count"] == 5
            assert "config" in data["data"]

    def test_get_grid_info_not_found(self):
        """Test getting info for non-existent grid."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/999")

            assert response.status_code == 404
            assert "not found" in response.json()["detail"]


# Due to size constraints, I'll create a comprehensive test file covering all endpoints.
# Let me continue with the remaining endpoint tests.


class TestAddTokenToGrid:
    """Test POST /grid/{grid_id}/tokens/{token_id} endpoint."""

    def test_add_token_success(self):
        """Test successfully adding token to grid."""
        app = FastAPI()

        mock_grid = Mock()
        mock_grid.__len__ = Mock(return_value=6)  # After adding

        mock_token = Mock()
        mock_token.id = 0x00001234

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid
        mock_grid_storage.add_token.return_value = True

        mock_token_storage = Mock()
        mock_token_storage.get.return_value = mock_token

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.post("/api/v1/grid/1/tokens/4660")  # 4660 = 0x1234

            assert response.status_code == 200
            data = response.json()
            assert data["success"] is True
            assert data["data"]["grid_id"] == 1
            assert data["data"]["token_id"] == 4660
            assert data["data"]["status"] == "added"
            assert data["data"]["grid_size"] == 6

    def test_add_token_grid_not_found(self):
        """Test adding token to non-existent grid."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = None

        mock_token_storage = Mock()

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.post("/api/v1/grid/999/tokens/1234")

            assert response.status_code == 404
            assert "Grid 999 not found" in response.json()["detail"]

    def test_add_token_token_not_found(self):
        """Test adding non-existent token to grid."""
        app = FastAPI()

        mock_grid = Mock()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid

        mock_token_storage = Mock()
        mock_token_storage.get.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_token_storage] = lambda: mock_token_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.post("/api/v1/grid/1/tokens/999999")

            assert response.status_code == 404
            assert "Token" in response.json()["detail"]
            assert "not found" in response.json()["detail"]


class TestRemoveTokenFromGrid:
    """Test DELETE /grid/{grid_id}/tokens/{token_id} endpoint."""

    def test_remove_token_success(self):
        """Test successfully removing token from grid."""
        app = FastAPI()

        mock_grid = Mock()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid
        mock_grid_storage.remove_token.return_value = True

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.delete("/api/v1/grid/1/tokens/1234")

            assert response.status_code == 204

    def test_remove_token_not_in_grid(self):
        """Test removing token that's not in grid."""
        app = FastAPI()

        mock_grid = Mock()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid
        mock_grid_storage.remove_token.return_value = False

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.delete("/api/v1/grid/1/tokens/999")

            assert response.status_code == 404
            assert "not found in grid" in response.json()["detail"]


class TestFindNeighbors:
    """Test GET /grid/{grid_id}/neighbors/{token_id} endpoint."""

    def test_find_neighbors_success(self):
        """Test successfully finding neighbors."""
        app = FastAPI()

        mock_grid = Mock()
        mock_grid.get = Mock(return_value=Mock())  # Token exists

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid
        mock_grid_storage.find_neighbors.return_value = [
            (100, 2.5),
            (200, 5.0),
            (300, 7.5)
        ]

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/1/neighbors/50?space=2&radius=10.0&max_results=10")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["grid_id"] == 1
            assert data["data"]["center_token_id"] == 50
            assert data["data"]["space"] == 2
            assert data["data"]["radius"] == 10.0
            assert data["data"]["count"] == 3
            assert len(data["data"]["neighbors"]) == 3

    def test_find_neighbors_token_not_in_grid(self):
        """Test finding neighbors for token not in grid."""
        app = FastAPI()

        mock_grid = Mock()
        mock_grid.get = Mock(return_value=None)  # Token doesn't exist

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/1/neighbors/999")

            assert response.status_code == 404
            assert "not found in grid" in response.json()["detail"]


class TestRangeQuery:
    """Test GET /grid/{grid_id}/range endpoint."""

    def test_range_query_success(self):
        """Test successful range query."""
        app = FastAPI()

        mock_grid = Mock()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid
        mock_grid_storage.range_query.return_value = [
            (100, 1.5),
            (200, 3.0)
        ]

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/1/range?space=0&x=1.0&y=2.0&z=3.0&radius=5.0")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["grid_id"] == 1
            assert data["data"]["space"] == 0
            assert data["data"]["center"] == [1.0, 2.0, 3.0]
            assert data["data"]["radius"] == 5.0
            assert data["data"]["count"] == 2

    def test_range_query_grid_not_found(self):
        """Test range query with non-existent grid."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = None

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/999/range")

            assert response.status_code == 404
            assert "not found" in response.json()["detail"]


class TestFieldInfluence:
    """Test GET /grid/{grid_id}/influence endpoint."""

    def test_field_influence_success(self):
        """Test successful field influence calculation."""
        app = FastAPI()

        mock_grid = Mock()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid
        mock_grid_storage.calculate_field_influence.return_value = 0.75

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/1/influence?space=4&x=10.0&y=20.0&z=30.0&radius=15.0")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["grid_id"] == 1
            assert data["data"]["space"] == 4
            assert data["data"]["position"] == [10.0, 20.0, 30.0]
            assert data["data"]["radius"] == 15.0
            assert data["data"]["influence"] == 0.75


class TestDensity:
    """Test GET /grid/{grid_id}/density endpoint."""

    def test_density_success(self):
        """Test successful density calculation."""
        app = FastAPI()

        mock_grid = Mock()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid
        mock_grid_storage.calculate_density.return_value = 0.85
        mock_grid_storage.range_query.return_value = [(1, 0.5), (2, 1.0), (3, 1.5)]

        mock_user = Mock()
        mock_user.scopes = ["grid:read"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.get("/api/v1/grid/1/density?space=3&x=5.0&y=5.0&z=5.0&radius=8.0")

            assert response.status_code == 200
            data = response.json()
            assert data["data"]["grid_id"] == 1
            assert data["data"]["density"] == 0.85
            assert data["data"]["tokens_in_range"] == 3


class TestDeleteGrid:
    """Test DELETE /grid/{grid_id} endpoint."""

    def test_delete_grid_success(self):
        """Test successful grid deletion."""
        app = FastAPI()

        mock_grid = Mock()

        mock_grid_storage = Mock()
        mock_grid_storage.get_grid.return_value = mock_grid
        mock_grid_storage.delete_grid.return_value = True

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.delete("/api/v1/grid/1")

            assert response.status_code == 204

    def test_delete_grid_not_found(self):
        """Test deleting non-existent grid."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_grid_storage.delete_grid.return_value = False  # Grid not found

        mock_user = Mock()
        mock_user.scopes = ["grid:write"]

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True), \
             patch('api.routers.grid.check_grid_available', return_value=True):
            client = TestClient(app)
            response = client.delete("/api/v1/grid/999")

            assert response.status_code == 404
            assert "not found" in response.json()["detail"]

    def test_delete_grid_without_permission(self):
        """Test grid deletion without permission."""
        app = FastAPI()

        mock_grid_storage = Mock()
        mock_user = Mock()
        mock_user.scopes = ["grid:read"]  # Has read but not write

        app.dependency_overrides[get_grid_storage] = lambda: mock_grid_storage
        app.dependency_overrides[get_current_active_user] = lambda: mock_user

        app.include_router(grid_router, prefix="/api/v1")

        with patch('api.routers.grid.settings.GRID_ENABLED', True), \
             patch('api.routers.grid.settings.ENABLE_NEW_GRID_API', True):
            client = TestClient(app)
            response = client.delete("/api/v1/grid/1")

            assert response.status_code == 403
            assert "grid:write" in response.json()["detail"]
