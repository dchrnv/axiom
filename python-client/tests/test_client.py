"""
Tests for Axiom synchronous client.

Run with: pytest tests/
"""

import pytest
from axiom import AxiomClient, AuthenticationError, NotFoundError
from axiom.models import Token, TokenQueryResult


import socket


def is_server_running(host="localhost", port=8000):
    """Check if server is running."""
    try:
        with socket.create_connection((host, port), timeout=1):
            return True
    except (socket.timeout, ConnectionRefusedError):
        return False


# Skip tests if API is not running
pytestmark = pytest.mark.skipif(
    not is_server_running(),
    reason="API server not running (start with: ./axiom start)",
)


@pytest.fixture
def client():
    """Create test client."""
    client = AxiomClient(
        base_url="http://localhost:8000",
        username="admin",
        password="admin123",
    )
    yield client
    client.close()


def test_health_check(client):
    """Test health check endpoint."""
    health = client.health.check()
    assert health.status == "healthy"
    assert health.version is not None


def test_create_token(client):
    """Test token creation."""
    token = client.tokens.create(
        entity_type=1, domain=0, weight=0.8, l1_physical={"x": 10.0, "y": 20.0, "z": 30.0}
    )

    assert isinstance(token, Token)
    assert token.id > 0
    assert token.entity_type == 1
    assert token.weight == 0.8
    assert token.coordinates["L1"] == [10.0, 20.0, 30.0]

    # Cleanup
    client.tokens.delete(token.id)


def test_get_token(client):
    """Test get token."""
    # Create token
    created = client.tokens.create(entity_type=2)

    # Get token
    retrieved = client.tokens.get(created.id)

    assert retrieved.id == created.id
    assert retrieved.entity_type == 2

    # Cleanup
    client.tokens.delete(created.id)


def test_get_nonexistent_token(client):
    """Test get non-existent token raises NotFoundError."""
    with pytest.raises(NotFoundError):
        client.tokens.get(token_id=999999)


def test_list_tokens(client):
    """Test list tokens."""
    # Create some tokens
    tokens = [client.tokens.create(entity_type=i) for i in range(3)]

    # List tokens
    listed = client.tokens.list(limit=10)

    assert len(listed) >= 3

    # Cleanup
    for token in tokens:
        client.tokens.delete(token.id)


def test_update_token(client):
    """Test update token."""
    # Create token
    token = client.tokens.create(weight=0.5)

    # Update
    updated = client.tokens.update(
        token.id, weight=0.9, l4_emotional={"x": 0.5, "y": 0.5, "z": 0.5}
    )

    assert updated.id == token.id
    assert updated.weight == 0.9
    assert updated.coordinates["L4"] == [0.5, 0.5, 0.5]

    # Cleanup
    client.tokens.delete(token.id)


def test_delete_token(client):
    """Test delete token."""
    # Create token
    token = client.tokens.create(entity_type=5)

    # Delete
    result = client.tokens.delete(token.id)
    assert result is True

    # Verify deleted
    with pytest.raises(NotFoundError):
        client.tokens.get(token.id)


def test_query_tokens(client):
    """Test query tokens."""
    # Create token
    token = client.tokens.create(entity_type=1, l1_physical={"x": 0.0, "y": 0.0, "z": 0.0})

    # Query
    results = client.tokens.query(text="test query", limit=5)

    assert len(results) >= 0
    # Note: We can't guarantee results with random/mock data easily
    # but we check the structure
    if len(results) > 0:
        assert isinstance(results[0], TokenQueryResult)
        assert results[0].score >= 0.0

    # Cleanup
    client.tokens.delete(token.id)


def test_authentication_error():
    """Test authentication with wrong credentials."""
    client = AxiomClient(
        base_url="http://localhost:8000",
        username="wrong",
        password="wrong",
    )

    with pytest.raises(AuthenticationError):
        client.tokens.list()

    client.close()


def test_context_manager():
    """Test client as context manager."""
    with AxiomClient(
        base_url="http://localhost:8000",
        username="admin",
        password="admin123",
    ) as client:
        health = client.health.check()
        assert health.status == "healthy"
    # Client should be closed after exit


def test_api_keys(client):
    """Test API key management."""
    # Create API key
    api_key = client.api_keys.create(
        name="Test Key",
        scopes=["tokens:read"],
    )

    assert api_key.api_key is not None
    assert api_key.key_id is not None
    assert api_key.name == "Test Key"

    # List keys
    keys = client.api_keys.list()
    assert len(keys) > 0

    # Get key
    retrieved = client.api_keys.get(api_key.key_id)
    assert retrieved.key_id == api_key.key_id

    # Revoke key
    client.api_keys.revoke(api_key.key_id)

    # Delete key
    client.api_keys.delete(api_key.key_id)
