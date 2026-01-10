"""
Unit tests for Query Router.

Tests semantic query execution endpoint.
"""

import pytest
from unittest.mock import Mock, patch
from fastapi import FastAPI
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

import api.routers.query as query
query_router = query.router


class TestQueryEndpoint:
    """Test POST /query endpoint."""

    def test_query_success(self):
        """Test successful query execution."""
        app = FastAPI()

        # Mock runtime
        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 5,
                "include_connections": False
            }
        )

        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert "data" in data
        assert data["data"]["query_text"] == "test query"
        assert "signal_id" in data["data"]
        assert "processing_time_ms" in data["data"]
        assert "tokens" in data["data"]
        assert isinstance(data["data"]["tokens"], list)

    def test_query_with_limit(self):
        """Test query respects limit parameter."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 3,
                "include_connections": False
            }
        )

        assert response.status_code == 200
        data = response.json()
        assert len(data["data"]["tokens"]) <= 3

    def test_query_with_connections(self):
        """Test query with include_connections flag."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 5,
                "include_connections": True
            }
        )

        assert response.status_code == 200
        data = response.json()
        # With include_connections=True, connections should be None (mock behavior)
        assert data["data"]["connections"] is None

    def test_query_without_connections(self):
        """Test query without connections returns empty list."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 5,
                "include_connections": False
            }
        )

        assert response.status_code == 200
        data = response.json()
        assert data["data"]["connections"] == []

    def test_query_runtime_not_initialized(self):
        """Test query fails when runtime not initialized."""
        app = FastAPI()

        # Override with None to simulate uninitialized runtime
        app.dependency_overrides[query.get_runtime] = lambda: None

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 5,
                "include_connections": False
            }
        )

        assert response.status_code == 503
        data = response.json()
        assert "Runtime not initialized" in data["detail"]

    def test_query_returns_token_results(self):
        """Test query returns properly structured token results."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "concept search",
                "limit": 5,
                "include_connections": False
            }
        )

        assert response.status_code == 200
        data = response.json()
        tokens = data["data"]["tokens"]

        # Check token structure
        assert len(tokens) > 0
        for token in tokens:
            assert "label" in token
            assert "score" in token
            assert "entity_type" in token
            assert token["entity_type"] == "Concept"

    def test_query_scores_decreasing(self):
        """Test query results have decreasing scores."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 5,
                "include_connections": False
            }
        )

        assert response.status_code == 200
        data = response.json()
        tokens = data["data"]["tokens"]

        if len(tokens) > 1:
            scores = [t["score"] for t in tokens]
            # Scores should be in descending order
            assert scores == sorted(scores, reverse=True)

    def test_query_includes_metadata(self):
        """Test query response includes all required metadata."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 5,
                "include_connections": False
            }
        )

        assert response.status_code == 200
        data = response.json()
        query_data = data["data"]

        # Check all required metadata fields
        assert "signal_id" in query_data
        assert "query_text" in query_data
        assert "processing_time_ms" in query_data
        assert "confidence" in query_data
        assert "interpretation" in query_data
        assert "total_candidates" in query_data

        # Verify values
        assert query_data["query_text"] == "test query"
        assert query_data["interpretation"] == "semantic_query"
        assert isinstance(query_data["confidence"], float)
        assert query_data["confidence"] == 0.85

    def test_query_processing_time_measured(self):
        """Test query measures processing time."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 5,
                "include_connections": False
            }
        )

        assert response.status_code == 200
        data = response.json()

        # Processing time should be present and positive
        assert "processing_time_ms" in data["data"]
        assert data["data"]["processing_time_ms"] > 0

        # Meta-level processing time should match
        assert "meta" in data
        assert "processing_time_ms" in data["meta"]
        assert data["meta"]["processing_time_ms"] == data["data"]["processing_time_ms"]

    def test_query_signal_id_unique(self):
        """Test each query generates unique signal ID."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)

        # Execute two queries
        response1 = client.post(
            "/api/v1/query",
            json={"text": "query 1", "limit": 5, "include_connections": False}
        )
        response2 = client.post(
            "/api/v1/query",
            json={"text": "query 2", "limit": 5, "include_connections": False}
        )

        assert response1.status_code == 200
        assert response2.status_code == 200

        signal_id1 = response1.json()["data"]["signal_id"]
        signal_id2 = response2.json()["data"]["signal_id"]

        # Signal IDs should be different
        assert signal_id1 != signal_id2

    def test_query_invalid_request_format(self):
        """Test query with invalid request format."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={"invalid": "data"}
        )

        # Should fail validation
        assert response.status_code == 422

    def test_query_empty_text(self):
        """Test query with empty text string."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "",
                "limit": 5,
                "include_connections": False
            }
        )

        # Empty text might be accepted but should return results
        # (Depends on validation - if accepted, should return 200)
        assert response.status_code in [200, 422]

    def test_query_large_limit(self):
        """Test query with large limit fails validation."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 1000,
                "include_connections": False
            }
        )

        # Large limit should fail validation (limit has max constraint)
        assert response.status_code == 422

    def test_query_total_candidates_matches_results(self):
        """Test total_candidates field matches number of results."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/query",
            json={
                "text": "test query",
                "limit": 5,
                "include_connections": False
            }
        )

        assert response.status_code == 200
        data = response.json()

        # total_candidates should match the number of tokens
        assert data["data"]["total_candidates"] == len(data["data"]["tokens"])


class TestQueryIntegration:
    """Integration tests for query endpoint."""

    def test_query_full_workflow(self):
        """Test complete query workflow."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)

        # Execute query
        response = client.post(
            "/api/v1/query",
            json={
                "text": "machine learning concepts",
                "limit": 3,
                "include_connections": False
            }
        )

        # Verify response structure
        assert response.status_code == 200
        data = response.json()

        # Top-level response
        assert data["success"] is True
        assert "data" in data
        assert "meta" in data
        assert "processing_time_ms" in data["meta"]

        # Query data
        query_data = data["data"]
        assert query_data["query_text"] == "machine learning concepts"
        assert len(query_data["tokens"]) <= 3
        assert query_data["interpretation"] == "semantic_query"
        assert isinstance(query_data["signal_id"], str)

        # Token results
        for token in query_data["tokens"]:
            assert "label" in token
            assert "score" in token
            assert 0 <= token["score"] <= 1

    def test_query_different_parameters(self):
        """Test query with various parameter combinations."""
        app = FastAPI()

        mock_runtime = Mock()
        app.dependency_overrides[query.get_runtime] = lambda: mock_runtime

        app.include_router(query_router, prefix="/api/v1")

        client = TestClient(app)

        test_cases = [
            {"text": "short", "limit": 1, "include_connections": False},
            {"text": "medium length query", "limit": 5, "include_connections": True},
            {"text": "a very long query text with multiple words", "limit": 10, "include_connections": False},
        ]

        for params in test_cases:
            response = client.post("/api/v1/query", json=params)
            assert response.status_code == 200
            data = response.json()
            assert data["data"]["query_text"] == params["text"]
            assert len(data["data"]["tokens"]) <= min(params["limit"], 5)
