"""
Unit tests for WebSocket Router.

Tests WebSocket management endpoints.
"""

import pytest
from unittest.mock import Mock, patch, AsyncMock
from fastapi import FastAPI
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

import api.routers.websocket as websocket
websocket_router = websocket.router


class TestGetWebSocketStatus:
    """Test GET /websocket/status endpoint."""

    def test_get_status_success(self):
        """Test successful status retrieval."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_stats = {
            "total_connections": 5,
            "total_channels": 3,
            "total_subscriptions": 10,
            "buffered_events": 2
        }

        with patch('api.routers.websocket.connection_manager.get_connection_stats', return_value=mock_stats):
            client = TestClient(app)
            response = client.get("/api/v1/websocket/status")

            assert response.status_code == 200
            data = response.json()
            assert data["status"] == "ok"
            assert "websocket" in data
            assert data["websocket"]["endpoint"] == "/ws"
            assert data["websocket"]["protocol"] == "ws"
            assert data["websocket"]["total_connections"] == 5
            assert data["websocket"]["total_channels"] == 3

    def test_get_status_no_connections(self):
        """Test status with no active connections."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_stats = {
            "total_connections": 0,
            "total_channels": 0,
            "total_subscriptions": 0,
            "buffered_events": 0
        }

        with patch('api.routers.websocket.connection_manager.get_connection_stats', return_value=mock_stats):
            client = TestClient(app)
            response = client.get("/api/v1/websocket/status")

            assert response.status_code == 200
            data = response.json()
            assert data["websocket"]["total_connections"] == 0
            assert data["websocket"]["total_channels"] == 0

    def test_get_status_includes_endpoint_info(self):
        """Test that status includes endpoint information."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_stats = {"total_connections": 1}

        with patch('api.routers.websocket.connection_manager.get_connection_stats', return_value=mock_stats):
            client = TestClient(app)
            response = client.get("/api/v1/websocket/status")

            data = response.json()
            assert "endpoint" in data["websocket"]
            assert "protocol" in data["websocket"]


class TestGetChannels:
    """Test GET /websocket/channels endpoint."""

    def test_get_channels_success(self):
        """Test successful channels retrieval."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_channels = ["system", "tokens", "queries"]
        mock_info = {
            "system": "System notifications",
            "tokens": "Token updates",
            "queries": "Query results"
        }

        with patch('api.routers.websocket.get_all_channels', return_value=mock_channels):
            with patch('api.routers.websocket.get_channel_info', return_value=mock_info):
                client = TestClient(app)
                response = client.get("/api/v1/websocket/channels")

                assert response.status_code == 200
                data = response.json()
                assert "channels" in data
                assert "descriptions" in data
                assert data["channels"] == mock_channels
                assert data["descriptions"] == mock_info

    def test_get_channels_empty(self):
        """Test channels retrieval with no channels."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        with patch('api.routers.websocket.get_all_channels', return_value=[]):
            with patch('api.routers.websocket.get_channel_info', return_value={}):
                client = TestClient(app)
                response = client.get("/api/v1/websocket/channels")

                assert response.status_code == 200
                data = response.json()
                assert data["channels"] == []
                assert data["descriptions"] == {}

    def test_get_channels_includes_descriptions(self):
        """Test that channels include descriptions."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_channels = ["test_channel"]
        mock_info = {"test_channel": "Test channel description"}

        with patch('api.routers.websocket.get_all_channels', return_value=mock_channels):
            with patch('api.routers.websocket.get_channel_info', return_value=mock_info):
                client = TestClient(app)
                response = client.get("/api/v1/websocket/channels")

                data = response.json()
                assert "test_channel" in data["descriptions"]
                assert data["descriptions"]["test_channel"] == "Test channel description"


class TestGetChannelStatus:
    """Test GET /websocket/channels/{channel} endpoint."""

    def test_get_channel_status_success(self):
        """Test successful channel status retrieval."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_subscribers = {"client_1", "client_2", "client_3"}

        with patch('api.routers.websocket.connection_manager.get_channel_subscribers', return_value=mock_subscribers):
            client = TestClient(app)
            response = client.get("/api/v1/websocket/channels/tokens")

            assert response.status_code == 200
            data = response.json()
            assert data["channel"] == "tokens"
            assert data["subscribers"] == 3
            assert len(data["subscriber_ids"]) == 3

    def test_get_channel_status_no_subscribers(self):
        """Test channel status with no subscribers."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        with patch('api.routers.websocket.connection_manager.get_channel_subscribers', return_value=set()):
            client = TestClient(app)
            response = client.get("/api/v1/websocket/channels/empty_channel")

            assert response.status_code == 200
            data = response.json()
            assert data["channel"] == "empty_channel"
            assert data["subscribers"] == 0
            assert data["subscriber_ids"] == []

    def test_get_channel_status_subscriber_ids(self):
        """Test that subscriber IDs are returned as list."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_subscribers = {"sub_123", "sub_456"}

        with patch('api.routers.websocket.connection_manager.get_channel_subscribers', return_value=mock_subscribers):
            client = TestClient(app)
            response = client.get("/api/v1/websocket/channels/test")

            data = response.json()
            assert isinstance(data["subscriber_ids"], list)
            assert set(data["subscriber_ids"]) == mock_subscribers


class TestBroadcastMessage:
    """Test POST /websocket/broadcast endpoint."""

    @pytest.mark.asyncio
    async def test_broadcast_success(self):
        """Test successful message broadcast."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_subscribers = {"client_1", "client_2"}
        mock_broadcast = AsyncMock()

        with patch('api.routers.websocket.connection_manager.get_channel_subscribers', return_value=mock_subscribers):
            with patch('api.routers.websocket.connection_manager.broadcast_to_channel', new=mock_broadcast):
                client = TestClient(app)
                response = client.post(
                    "/api/v1/websocket/broadcast?channel=tokens",
                    json={"type": "update", "data": {"token_id": 123}}
                )

                assert response.status_code == 200
                data = response.json()
                assert data["status"] == "ok"
                assert data["channel"] == "tokens"
                assert data["subscribers"] == 2
                assert data["message_sent"] is True

                # Verify broadcast was called
                mock_broadcast.assert_awaited_once()
                call_args = mock_broadcast.call_args
                assert call_args[0][0] == "tokens"  # channel
                assert "channel" in call_args[0][1]  # event has channel field

    @pytest.mark.asyncio
    async def test_broadcast_no_subscribers(self):
        """Test broadcast with no subscribers."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_broadcast = AsyncMock()

        with patch('api.routers.websocket.connection_manager.get_channel_subscribers', return_value=set()):
            with patch('api.routers.websocket.connection_manager.broadcast_to_channel', new=mock_broadcast):
                client = TestClient(app)
                response = client.post(
                    "/api/v1/websocket/broadcast?channel=empty",
                    json={"type": "test"}
                )

                assert response.status_code == 200
                data = response.json()
                assert data["subscribers"] == 0
                assert data["message_sent"] is True

    @pytest.mark.asyncio
    async def test_broadcast_includes_channel_in_event(self):
        """Test that broadcast includes channel in event message."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_subscribers = {"client_1"}
        mock_broadcast = AsyncMock()

        with patch('api.routers.websocket.connection_manager.get_channel_subscribers', return_value=mock_subscribers):
            with patch('api.routers.websocket.connection_manager.broadcast_to_channel', new=mock_broadcast):
                client = TestClient(app)
                response = client.post(
                    "/api/v1/websocket/broadcast?channel=test_channel",
                    json={"type": "notification", "message": "Hello"}
                )

                assert response.status_code == 200

                # Verify event structure
                call_args = mock_broadcast.call_args
                event = call_args[0][1]
                assert event["channel"] == "test_channel"
                assert event["type"] == "notification"
                assert event["message"] == "Hello"


class TestWebSocketIntegration:
    """Test WebSocket endpoints working together."""

    def test_status_and_channels_consistency(self):
        """Test that status and channels provide consistent data."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_stats = {"total_channels": 3}
        mock_channels = ["system", "tokens", "queries"]

        with patch('api.routers.websocket.connection_manager.get_connection_stats', return_value=mock_stats):
            with patch('api.routers.websocket.get_all_channels', return_value=mock_channels):
                with patch('api.routers.websocket.get_channel_info', return_value={}):
                    client = TestClient(app)

                    # Get status
                    status_response = client.get("/api/v1/websocket/status")
                    status_data = status_response.json()

                    # Get channels
                    channels_response = client.get("/api/v1/websocket/channels")
                    channels_data = channels_response.json()

                    # Both should succeed
                    assert status_response.status_code == 200
                    assert channels_response.status_code == 200

                    # Data should be consistent
                    assert status_data["websocket"]["total_channels"] == 3
                    assert len(channels_data["channels"]) == 3

    @pytest.mark.asyncio
    async def test_channel_status_after_broadcast(self):
        """Test getting channel status after broadcasting."""
        app = FastAPI()
        app.include_router(websocket_router, prefix="/api/v1")

        mock_subscribers = {"client_1", "client_2"}
        mock_broadcast = AsyncMock()

        with patch('api.routers.websocket.connection_manager.get_channel_subscribers', return_value=mock_subscribers):
            with patch('api.routers.websocket.connection_manager.broadcast_to_channel', new=mock_broadcast):
                client = TestClient(app)

                # Broadcast message
                broadcast_response = client.post(
                    "/api/v1/websocket/broadcast?channel=tokens",
                    json={"type": "update"}
                )

                # Get channel status
                status_response = client.get("/api/v1/websocket/channels/tokens")

                # Both should succeed
                assert broadcast_response.status_code == 200
                assert status_response.status_code == 200

                # Should show same subscriber count
                broadcast_data = broadcast_response.json()
                status_data = status_response.json()
                assert broadcast_data["subscribers"] == status_data["subscribers"]
