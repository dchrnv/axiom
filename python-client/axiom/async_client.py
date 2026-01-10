"""
Asynchronous Axiom Client

Async client class for interacting with Axiom REST API.
"""

from typing import Optional, List, Dict, Any
import httpx

from .exceptions import (
    AxiomError,
    AuthenticationError,
    AuthorizationError,
    NotFoundError,
    ValidationError,
    RateLimitError,
    ServerError,
    ConnectionError as ClientConnectionError,
    TimeoutError as ClientTimeoutError,
)
from .models import (
    Token,
    APIKey,
    APIKeyCreated,
    HealthStatus,
    SystemStatus,
    TokenQueryResult,
)


class AsyncAxiomClient:
    """
    Asynchronous client for Axiom REST API.

    Example:
        >>> async with AsyncAxiomClient(
        ...     base_url="http://localhost:8000",
        ...     api_key="ng_1234567890abcdef"
        ... ) as client:
        ...     token = await client.tokens.create(text="hello world")
        ...     results = await client.tokens.query(query_vector=[...])

    Args:
        base_url: Base URL of Axiom API
        username: Username for JWT authentication
        password: Password for JWT authentication
        api_key: API key for authentication
        timeout: Request timeout in seconds (default: 30)
        verify_ssl: Verify SSL certificates (default: True)
    """

    def __init__(
        self,
        base_url: str,
        username: Optional[str] = None,
        password: Optional[str] = None,
        api_key: Optional[str] = None,
        timeout: float = 30.0,
        verify_ssl: bool = True,
    ):
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self.api_key = api_key
        self.username = username
        self.password = password

        # Initialize HTTP client
        self._client = httpx.AsyncClient(
            timeout=timeout,
            verify=verify_ssl,
            follow_redirects=True,
        )

        # JWT state
        self._access_token: Optional[str] = None

        # Initialize resource clients
        self.tokens = AsyncTokensClient(self)
        self.api_keys = AsyncAPIKeysClient(self)
        self.health = AsyncHealthClient(self)

    async def _get_auth_headers(self) -> Dict[str, str]:
        """Get authentication headers."""
        if self.api_key:
            return {"Authorization": f"Bearer {self.api_key}"}

        if self._access_token:
            return {"Authorization": f"Bearer {self._access_token}"}

        # Need to login
        if self.username and self.password:
            await self._login()
            return {"Authorization": f"Bearer {self._access_token}"}

        raise AuthenticationError("No authentication credentials provided")

    async def _login(self):
        """Perform JWT login."""
        response = await self._client.post(
            f"{self.base_url}/api/v1/auth/login",
            json={"username": self.username, "password": self.password},
        )

        if response.status_code != 200:
            raise AuthenticationError(f"Login failed: {response.text}")

        data = response.json()
        self._access_token = data["access_token"]

    async def _request(
        self,
        method: str,
        path: str,
        **kwargs,
    ) -> httpx.Response:
        """Make HTTP request with authentication."""
        url = f"{self.base_url}{path}"

        # Add authentication headers
        headers = kwargs.pop("headers", {})
        auth_headers = await self._get_auth_headers()
        headers.update(auth_headers)

        try:
            response = await self._client.request(
                method=method,
                url=url,
                headers=headers,
                **kwargs,
            )

            # Handle errors
            if response.status_code >= 400:
                self._handle_error_response(response)

            return response

        except httpx.TimeoutException as e:
            raise ClientTimeoutError(f"Request timeout: {e}")
        except httpx.ConnectError as e:
            raise ClientConnectionError(f"Connection failed: {e}")
        except Exception as e:
            if isinstance(e, AxiomError):
                raise
            raise AxiomError(f"Request failed: {e}")

    def _handle_error_response(self, response: httpx.Response):
        """Handle error responses from API."""
        try:
            data = response.json()
            error = data.get("error", {})
            message = error.get("message", response.text)
            error_code = error.get("code")
            details = error.get("details", {})
        except Exception:
            message = response.text
            error_code = None
            details = {}

        status_code = response.status_code

        if status_code == 401:
            raise AuthenticationError(message, status_code, error_code, details)
        elif status_code == 403:
            raise AuthorizationError(message, status_code, error_code, details)
        elif status_code == 404:
            raise NotFoundError(message, status_code, error_code, details)
        elif status_code == 422:
            raise ValidationError(message, status_code, error_code, details)
        elif status_code == 429:
            retry_after = response.headers.get("Retry-After")
            raise RateLimitError(
                message,
                status_code,
                error_code,
                details,
                retry_after=int(retry_after) if retry_after else None,
            )
        elif status_code >= 500:
            raise ServerError(message, status_code, error_code, details)
        else:
            raise AxiomError(message, status_code, error_code, details)

    async def close(self):
        """Close the HTTP client."""
        await self._client.aclose()

    async def __aenter__(self):
        """Async context manager entry."""
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit."""
        await self.close()


class AsyncTokensClient:
    """Async client for token operations."""

    def __init__(self, client: AsyncAxiomClient):
        self._client = client

    async def create(
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
        """Create a new token."""
        payload = {
            "entity_type": entity_type,
            "domain": domain,
            "weight": weight,
            "field_radius": field_radius,
            "field_strength": field_strength,
            "persistent": persistent,
        }
        if l1_physical:
            payload["l1_physical"] = l1_physical
        if l2_sensory:
            payload["l2_sensory"] = l2_sensory
        if l3_motor:
            payload["l3_motor"] = l3_motor
        if l4_emotional:
            payload["l4_emotional"] = l4_emotional
        if l5_cognitive:
            payload["l5_cognitive"] = l5_cognitive
        if l6_social:
            payload["l6_social"] = l6_social
        if l7_temporal:
            payload["l7_temporal"] = l7_temporal
        if l8_abstract:
            payload["l8_abstract"] = l8_abstract

        response = await self._client._request(
            "POST",
            "/api/v1/tokens",
            json=payload,
        )
        data = response.json().get("data", {})
        return Token(**data)

    async def get(self, token_id: int) -> Token:
        """Get token by ID."""
        response = await self._client._request("GET", f"/api/v1/tokens/{token_id}")
        data = response.json().get("data", {})
        return Token(**data)

    async def list(self, limit: int = 100, offset: int = 0) -> List[Token]:
        """List tokens."""
        response = await self._client._request(
            "GET",
            "/api/v1/tokens",
            params={"limit": limit, "offset": offset},
        )
        data = response.json().get("data", {})
        return [Token(**item) for item in data.get("tokens", [])]

    async def update(
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
        """Update token."""
        update_data = {}
        if weight is not None:
            update_data["weight"] = weight
        if field_radius is not None:
            update_data["field_radius"] = field_radius
        if field_strength is not None:
            update_data["field_strength"] = field_strength
        if l1_physical:
            update_data["l1_physical"] = l1_physical
        if l2_sensory:
            update_data["l2_sensory"] = l2_sensory
        if l3_motor:
            update_data["l3_motor"] = l3_motor
        if l4_emotional:
            update_data["l4_emotional"] = l4_emotional
        if l5_cognitive:
            update_data["l5_cognitive"] = l5_cognitive
        if l6_social:
            update_data["l6_social"] = l6_social
        if l7_temporal:
            update_data["l7_temporal"] = l7_temporal
        if l8_abstract:
            update_data["l8_abstract"] = l8_abstract

        response = await self._client._request(
            "PUT",
            f"/api/v1/tokens/{token_id}",
            json=update_data,
        )
        data = response.json().get("data", {})
        return Token(**data)

    async def delete(self, token_id: int) -> bool:
        """Delete token."""
        await self._client._request("DELETE", f"/api/v1/tokens/{token_id}")
        return True

    async def query(
        self,
        text: str,
        limit: int = 10,
        threshold: float = 0.0,
        spaces: Optional[List[str]] = None,
        include_connections: bool = False,
    ) -> List[TokenQueryResult]:
        """Query similar tokens."""
        query_data = {
            "text": text,
            "limit": limit,
            "threshold": threshold,
            "include_connections": include_connections,
        }
        if spaces:
            query_data["spaces"] = spaces

        response = await self._client._request(
            "POST",
            "/api/v1/query",
            json=query_data,
        )
        data = response.json().get("data", {})
        return [TokenQueryResult(**item) for item in data.get("tokens", [])]


class AsyncAPIKeysClient:
    """Async client for API key operations."""

    def __init__(self, client: AsyncAxiomClient):
        self._client = client

    async def create(
        self,
        name: str,
        scopes: Optional[List[str]] = None,
        expires_in_days: Optional[int] = None,
    ) -> APIKeyCreated:
        """Create a new API key."""
        response = await self._client._request(
            "POST",
            "/api/v1/api-keys",
            json={
                "name": name,
                "scopes": scopes,
                "expires_in_days": expires_in_days,
            },
        )
        data = response.json().get("data", {})
        return APIKeyCreated(**data)

    async def list(self) -> List[APIKey]:
        """List all API keys."""
        response = await self._client._request("GET", "/api/v1/api-keys")
        data = response.json().get("data", {})
        return [APIKey(**item) for item in data.get("keys", [])]

    async def get(self, key_id: str) -> APIKey:
        """Get API key by ID."""
        response = await self._client._request("GET", f"/api/v1/api-keys/{key_id}")
        data = response.json().get("data", {})
        return APIKey(**data)

    async def revoke(self, key_id: str) -> bool:
        """Revoke API key."""
        await self._client._request("POST", f"/api/v1/api-keys/{key_id}/revoke")
        return True

    async def delete(self, key_id: str) -> bool:
        """Delete API key."""
        await self._client._request("DELETE", f"/api/v1/api-keys/{key_id}")
        return True


class AsyncHealthClient:
    """Async client for health checks."""

    def __init__(self, client: AsyncAxiomClient):
        self._client = client

    async def check(self) -> HealthStatus:
        """Check API health."""
        response = await self._client._request("GET", "/api/v1/health")
        data = response.json().get("data", {})
        return HealthStatus(**data)

    async def status(self) -> SystemStatus:
        """Get system status."""
        response = await self._client._request("GET", "/api/v1/status")
        data = response.json().get("data", {})
        return SystemStatus(**data)
