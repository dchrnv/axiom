"""
Testing utilities and mock clients for Axiom.

Provides mock clients and test fixtures for unit testing applications
that use Axiom without requiring a live API server.
"""

from typing import List, Optional, Dict
from datetime import datetime
import random

from .models import (
    Token,
    APIKey,
    APIKeyCreated,
    HealthStatus,
    SystemStatus,
    TokenQueryResult,
)


class MockAxiomClient:
    """
    Mock Axiom client for testing.

    Simulates Axiom API responses without requiring a live server.
    Useful for unit tests and CI/CD pipelines.

    Example:
        >>> from axiom.testing import MockAxiomClient
        >>> client = MockAxiomClient()
        >>> token = client.tokens.create(text="test")
        >>> assert token.id > 0
        >>> assert token.text == "test"
    """

    def __init__(self, **kwargs):
        """Initialize mock client."""
        self._tokens_store: Dict[int, Token] = {}
        self._api_keys_store: Dict[str, APIKey] = {}
        self._next_token_id = 1
        self._next_key_id = 1

        self.tokens = self.MockTokensClient(self)
        self.api_keys = self.MockAPIKeysClient(self)
        self.health = self.MockHealthClient(self)

    def close(self):
        """Close client (no-op for mock)."""
        pass

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    class MockTokensClient:
        """Mock tokens resource."""

        def __init__(self, parent):
            self.parent = parent

        def create(
            self,
            entity_type: int = 0,
            domain: int = 0,
            weight: float = 0.5,
            field_radius: float = 1.0,
            field_strength: float = 1.0,
            persistent: bool = False,
            l1_physical: Optional[Dict[str, float]] = None,
            l2_sensory: Optional[Dict[str, float]] = None,
            l3_motor: Optional[Dict[str, float]] = None,
            l4_emotional: Optional[Dict[str, float]] = None,
            l5_cognitive: Optional[Dict[str, float]] = None,
            l6_social: Optional[Dict[str, float]] = None,
            l7_temporal: Optional[Dict[str, float]] = None,
            l8_abstract: Optional[Dict[str, float]] = None,
        ) -> Token:
            """Create mock token."""
            token_id = self.parent._next_token_id
            self.parent._next_token_id += 1

            # Generate mock coordinates
            coordinates = {
                "L1": [
                    l1_physical.get("x", 0.0),
                    l1_physical.get("y", 0.0),
                    l1_physical.get("z", 0.0),
                ]
                if l1_physical
                else None,
                "L2": [l2_sensory.get("x", 0.0), l2_sensory.get("y", 0.0), l2_sensory.get("z", 0.0)]
                if l2_sensory
                else None,
                "L3": [l3_motor.get("x", 0.0), l3_motor.get("y", 0.0), l3_motor.get("z", 0.0)]
                if l3_motor
                else None,
                "L4": [
                    l4_emotional.get("x", 0.0),
                    l4_emotional.get("y", 0.0),
                    l4_emotional.get("z", 0.0),
                ]
                if l4_emotional
                else None,
                "L5": [
                    l5_cognitive.get("x", 0.0),
                    l5_cognitive.get("y", 0.0),
                    l5_cognitive.get("z", 0.0),
                ]
                if l5_cognitive
                else None,
                "L6": [l6_social.get("x", 0.0), l6_social.get("y", 0.0), l6_social.get("z", 0.0)]
                if l6_social
                else None,
                "L7": [
                    l7_temporal.get("x", 0.0),
                    l7_temporal.get("y", 0.0),
                    l7_temporal.get("z", 0.0),
                ]
                if l7_temporal
                else None,
                "L8": [
                    l8_abstract.get("x", 0.0),
                    l8_abstract.get("y", 0.0),
                    l8_abstract.get("z", 0.0),
                ]
                if l8_abstract
                else None,
            }

            token = Token(
                id=token_id,
                id_hex=f"0x{token_id:08X}",
                local_id=token_id,
                entity_type=entity_type,
                domain=domain,
                weight=weight,
                field_radius=field_radius,
                field_strength=field_strength,
                timestamp=int(datetime.now(datetime.UTC).timestamp()),
                age_seconds=0,
                flags={"active": True, "persistent": persistent},
                coordinates=coordinates,
            )

            self.parent._tokens_store[token_id] = token
            return token

        def get(self, token_id: int) -> Token:
            """Get mock token."""
            from .exceptions import NotFoundError

            if token_id not in self.parent._tokens_store:
                raise NotFoundError(f"Token {token_id} not found", "TOKEN_NOT_FOUND")

            return self.parent._tokens_store[token_id]

        def list(self, limit: int = 100, offset: int = 0) -> List[Token]:
            """List mock tokens."""
            tokens = list(self.parent._tokens_store.values())
            return tokens[offset : offset + limit]

        def update(
            self,
            token_id: int,
            weight: Optional[float] = None,
            field_radius: Optional[float] = None,
            field_strength: Optional[float] = None,
            l1_physical: Optional[Dict[str, float]] = None,
            l2_sensory: Optional[Dict[str, float]] = None,
            l3_motor: Optional[Dict[str, float]] = None,
            l4_emotional: Optional[Dict[str, float]] = None,
            l5_cognitive: Optional[Dict[str, float]] = None,
            l6_social: Optional[Dict[str, float]] = None,
            l7_temporal: Optional[Dict[str, float]] = None,
            l8_abstract: Optional[Dict[str, float]] = None,
        ) -> Token:
            """Update mock token."""
            token = self.get(token_id)

            if weight is not None:
                token.weight = weight
            if field_radius is not None:
                token.field_radius = field_radius
            if field_strength is not None:
                token.field_strength = field_strength

            # Update coordinates if provided
            if l1_physical:
                token.coordinates["L1"] = [
                    l1_physical.get("x", 0.0),
                    l1_physical.get("y", 0.0),
                    l1_physical.get("z", 0.0),
                ]
            if l2_sensory:
                token.coordinates["L2"] = [
                    l2_sensory.get("x", 0.0),
                    l2_sensory.get("y", 0.0),
                    l2_sensory.get("z", 0.0),
                ]
            if l3_motor:
                token.coordinates["L3"] = [
                    l3_motor.get("x", 0.0),
                    l3_motor.get("y", 0.0),
                    l3_motor.get("z", 0.0),
                ]
            if l4_emotional:
                token.coordinates["L4"] = [
                    l4_emotional.get("x", 0.0),
                    l4_emotional.get("y", 0.0),
                    l4_emotional.get("z", 0.0),
                ]
            if l5_cognitive:
                token.coordinates["L5"] = [
                    l5_cognitive.get("x", 0.0),
                    l5_cognitive.get("y", 0.0),
                    l5_cognitive.get("z", 0.0),
                ]
            if l6_social:
                token.coordinates["L6"] = [
                    l6_social.get("x", 0.0),
                    l6_social.get("y", 0.0),
                    l6_social.get("z", 0.0),
                ]
            if l7_temporal:
                token.coordinates["L7"] = [
                    l7_temporal.get("x", 0.0),
                    l7_temporal.get("y", 0.0),
                    l7_temporal.get("z", 0.0),
                ]
            if l8_abstract:
                token.coordinates["L8"] = [
                    l8_abstract.get("x", 0.0),
                    l8_abstract.get("y", 0.0),
                    l8_abstract.get("z", 0.0),
                ]

            self.parent._tokens_store[token_id] = token
            return token

        def delete(self, token_id: int) -> bool:
            """Delete mock token."""
            from .exceptions import NotFoundError

            if token_id not in self.parent._tokens_store:
                raise NotFoundError(f"Token {token_id} not found", "TOKEN_NOT_FOUND")

            del self.parent._tokens_store[token_id]
            return True

        def query(
            self,
            text: str,
            limit: int = 10,
            threshold: float = 0.0,
            spaces: Optional[List[str]] = None,
            include_connections: bool = False,
        ) -> List[TokenQueryResult]:
            """Query mock tokens."""
            # Simple mock: return random tokens with random similarities
            tokens = list(self.parent._tokens_store.values())

            # Limit to limit
            tokens = tokens[:limit]

            # Generate results with mock similarities
            results = []
            for token in tokens:
                similarity = random.uniform(0.7, 1.0)

                if threshold is None or similarity >= threshold:
                    results.append(
                        TokenQueryResult(
                            token_id=token.id,
                            label=f"token_{token.id}",
                            score=similarity,
                            entity_type="Concept",
                            coordinates=token.coordinates,
                        )
                    )

            # Sort by score descending
            results.sort(key=lambda r: r.score, reverse=True)
            return results

    class MockAPIKeysClient:
        """Mock API keys resource."""

        def __init__(self, parent):
            self.parent = parent

        def create(
            self, name: str, scopes: List[str], expires_in_days: Optional[int] = None
        ) -> APIKeyCreated:
            """Create mock API key."""
            key_id = f"key_{self.parent._next_key_id:08d}"
            self.parent._next_key_id += 1

            api_key_str = f"ng_mock_{key_id}_{random.randint(10000, 99999)}"

            expires_at = None
            if expires_in_days:
                from datetime import timedelta

                expires_at = (
                    datetime.now(datetime.UTC) + timedelta(days=expires_in_days)
                ).isoformat()

            # Store as APIKey
            api_key_obj = APIKey(
                key_id=key_id,
                name=name,
                key_prefix=api_key_str[:7],
                scopes=scopes,
                created_at=datetime.now(datetime.UTC).isoformat(),
                expires_at=expires_at,
                last_used_at=None,
                disabled=False,
            )
            self.parent._api_keys_store[key_id] = api_key_obj

            # Return as APIKeyCreated
            return APIKeyCreated(
                key_id=key_id,
                name=name,
                api_key=api_key_str,
                key_prefix=api_key_str[:7],
                scopes=scopes,
                created_at=api_key_obj.created_at,
                expires_at=expires_at,
            )

        def list(self) -> List[APIKey]:
            """List mock API keys."""
            # Return copies without api_key field
            return [
                APIKey(
                    key_id=k.key_id,
                    name=k.name,
                    key_prefix=k.key_prefix,
                    scopes=k.scopes,
                    created_at=k.created_at,
                    expires_at=k.expires_at,
                    last_used_at=k.last_used_at,
                    disabled=k.disabled,
                    api_key=None,
                )
                for k in self.parent._api_keys_store.values()
            ]

        def get(self, key_id: str) -> APIKey:
            """Get mock API key."""
            from .exceptions import NotFoundError

            if key_id not in self.parent._api_keys_store:
                raise NotFoundError(f"API key {key_id} not found", "API_KEY_NOT_FOUND")

            key = self.parent._api_keys_store[key_id]
            # Return without api_key field
            return APIKey(
                key_id=key.key_id,
                name=key.name,
                key_prefix=key.key_prefix,
                scopes=key.scopes,
                created_at=key.created_at,
                expires_at=key.expires_at,
                last_used_at=key.last_used_at,
                disabled=key.disabled,
                api_key=None,
            )

        def revoke(self, key_id: str) -> None:
            """Revoke mock API key."""
            self.get(key_id)
            self.parent._api_keys_store[key_id].disabled = True

        def delete(self, key_id: str) -> None:
            """Delete mock API key."""
            from .exceptions import NotFoundError

            if key_id not in self.parent._api_keys_store:
                raise NotFoundError(f"API key {key_id} not found", "API_KEY_NOT_FOUND")

            del self.parent._api_keys_store[key_id]

    class MockHealthClient:
        """Mock health resource."""

        def __init__(self, parent):
            self.parent = parent

        def check(self) -> HealthStatus:
            """Mock health check."""
            return HealthStatus(
                status="healthy",
                version="0.59.0-mock",
                timestamp=datetime.now(datetime.UTC).isoformat(),
            )

        def status(self) -> SystemStatus:
            """Mock system status."""
            return SystemStatus(
                state="running",
                uptime_seconds=12345.0,
                tokens={
                    "total": len(self.parent._tokens_store),
                    "active": len(self.parent._tokens_store),
                },
                connections={"total": 0, "active": 0},
                memory_usage_mb=128.5,
                cpu_usage_percent=5.0,
                components={
                    "runtime": "running",
                    "runtime_storage": "running",
                    "token_storage": "running",
                    "grid_storage": "running",
                    "cdna_storage": "running",
                },
                version="0.59.0-mock",
                storage_backend="runtime",
                cdna_profile="explorer",
            )


# Convenience fixtures for pytest
def mock_axiom_client(**kwargs):
    """
    Create a mock Axiom client for testing.

    Example:
        >>> def test_create_token():
        ...     client = mock_axiom_client()
        ...     token = client.tokens.create(text="test")
        ...     assert token.text == "test"
    """
    return MockAxiomClient(**kwargs)


def mock_token(**kwargs) -> Token:
    """
    Create a mock token for testing.

    Args:
        **kwargs: Override default token fields

    Example:
        >>> token = mock_token(text="custom text", id=123)
        >>> assert token.id == 123
        >>> assert token.text == "custom text"
    """
    defaults = {
        "id": 1,
        "id_hex": "0x00000001",
        "local_id": 1,
        "entity_type": 0,
        "domain": 0,
        "weight": 0.5,
        "field_radius": 1.0,
        "field_strength": 1.0,
        "timestamp": int(datetime.now(datetime.UTC).timestamp()),
        "age_seconds": 0,
        "flags": {"active": True, "persistent": False},
        "coordinates": {f"L{i + 1}": None for i in range(8)},
    }
    defaults.update(kwargs)
    return Token(**defaults)


def mock_api_key(**kwargs) -> APIKeyCreated:
    """
    Create a mock API key for testing.

    Args:
        **kwargs: Override default API key fields

    Example:
        >>> api_key = mock_api_key(name="test key", scopes=["tokens:read"])
        >>> assert api_key.name == "test key"
    """
    defaults = {
        "key_id": "key_00000001",
        "name": "mock api key",
        "key_prefix": "ng_mock",
        "scopes": ["tokens:read", "tokens:write"],
        "created_at": datetime.now(datetime.UTC).isoformat(),
        "expires_at": None,
        "last_used_at": None,
        "disabled": False,
        "api_key": "ng_mock_key_12345",
    }
    defaults.update(kwargs)
    return APIKeyCreated(**defaults)
