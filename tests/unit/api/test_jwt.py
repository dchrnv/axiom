"""
Unit tests for JWT token management.

Tests JWT token creation, validation, and refresh without external dependencies.
"""

import pytest
import jwt as pyjwt
from datetime import datetime, timedelta

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.auth.jwt import JWTManager, TokenPayload


class TestJWTTokenCreation:
    """Test JWT token creation."""

    def test_create_access_token(self):
        """Test creating access token."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_access_token("user123", ["tokens:read", "tokens:write"])

        assert isinstance(token, str)
        assert len(token) > 0

        # Decode and verify
        payload = pyjwt.decode(token, "test_secret", algorithms=["HS256"])
        assert payload["sub"] == "user123"
        assert payload["scopes"] == ["tokens:read", "tokens:write"]
        assert payload["token_type"] == "access"

    def test_create_access_token_without_scopes(self):
        """Test creating access token without scopes."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_access_token("user456")

        payload = pyjwt.decode(token, "test_secret", algorithms=["HS256"])
        assert payload["sub"] == "user456"
        assert payload["scopes"] == []

    def test_create_access_token_custom_expiration(self):
        """Test creating access token with custom expiration."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_access_token(
            "user789",
            expires_delta=timedelta(minutes=30)
        )

        payload = pyjwt.decode(token, "test_secret", algorithms=["HS256"])
        exp_time = datetime.fromtimestamp(payload["exp"])
        iat_time = datetime.fromtimestamp(payload["iat"])

        # Check expiration is ~30 minutes from now
        delta = (exp_time - iat_time).total_seconds()
        assert 1790 <= delta <= 1810  # 30 min ± 10 sec

    def test_create_refresh_token(self):
        """Test creating refresh token."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_refresh_token("user123")

        payload = pyjwt.decode(token, "test_secret", algorithms=["HS256"])
        assert payload["sub"] == "user123"
        assert payload["token_type"] == "refresh"
        assert payload["scopes"] == []  # Refresh tokens have no scopes

    def test_create_refresh_token_custom_expiration(self):
        """Test creating refresh token with custom expiration."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_refresh_token(
            "user123",
            expires_delta=timedelta(days=14)
        )

        payload = pyjwt.decode(token, "test_secret", algorithms=["HS256"])
        exp_time = datetime.fromtimestamp(payload["exp"])
        iat_time = datetime.fromtimestamp(payload["iat"])

        # Check expiration is ~14 days
        delta = (exp_time - iat_time).total_seconds()
        expected = 14 * 24 * 60 * 60  # 14 days in seconds
        assert expected - 10 <= delta <= expected + 10


class TestJWTTokenVerification:
    """Test JWT token verification."""

    def test_verify_valid_access_token(self):
        """Test verifying valid access token."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_access_token("user123", ["tokens:read"])

        payload = manager.verify_token(token, expected_type="access")
        assert isinstance(payload, TokenPayload)
        assert payload.sub == "user123"
        assert "tokens:read" in payload.scopes
        assert payload.token_type == "access"

    def test_verify_valid_refresh_token(self):
        """Test verifying valid refresh token."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_refresh_token("user456")

        payload = manager.verify_token(token, expected_type="refresh")
        assert isinstance(payload, TokenPayload)
        assert payload.sub == "user456"
        assert payload.token_type == "refresh"

    def test_verify_expired_token(self):
        """Test verifying expired token."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_access_token(
            "user123",
            expires_delta=timedelta(seconds=-1)  # Already expired
        )

        with pytest.raises(pyjwt.ExpiredSignatureError):
            manager.verify_token(token)

    def test_verify_invalid_signature(self):
        """Test verifying token with wrong signature."""
        manager1 = JWTManager(secret_key="secret1")
        manager2 = JWTManager(secret_key="secret2")

        token = manager1.create_access_token("user123")

        with pytest.raises(pyjwt.InvalidTokenError):
            manager2.verify_token(token)

    def test_verify_wrong_token_type(self):
        """Test verifying token with wrong type."""
        manager = JWTManager(secret_key="test_secret")
        access_token = manager.create_access_token("user123")

        # Try to verify access token as refresh token
        with pytest.raises(ValueError, match="Invalid token type"):
            manager.verify_token(access_token, expected_type="refresh")

    def test_verify_malformed_token(self):
        """Test verifying malformed token."""
        manager = JWTManager(secret_key="test_secret")

        with pytest.raises(pyjwt.InvalidTokenError):
            manager.verify_token("not.a.valid.token")


class TestJWTTokenRefresh:
    """Test JWT token refresh."""

    def test_refresh_access_token(self):
        """Test refreshing access token."""
        manager = JWTManager(secret_key="test_secret")
        refresh_token = manager.create_refresh_token("user123")

        tokens = manager.refresh_access_token(refresh_token)

        assert "access_token" in tokens
        assert "refresh_token" in tokens
        assert "token_type" in tokens
        assert tokens["token_type"] == "bearer"

        # Verify new access token
        access_payload = manager.verify_token(tokens["access_token"])
        assert access_payload.sub == "user123"

        # Verify new refresh token
        refresh_payload = manager.verify_token(
            tokens["refresh_token"],
            expected_type="refresh"
        )
        assert refresh_payload.sub == "user123"

    def test_refresh_with_access_token_fails(self):
        """Test that refreshing with access token fails."""
        manager = JWTManager(secret_key="test_secret")
        access_token = manager.create_access_token("user123")

        with pytest.raises(ValueError, match="Invalid token type"):
            manager.refresh_access_token(access_token)

    def test_refresh_with_expired_token_fails(self):
        """Test that refreshing with expired token fails."""
        manager = JWTManager(secret_key="test_secret")
        refresh_token = manager.create_refresh_token(
            "user123",
            expires_delta=timedelta(seconds=-1)
        )

        with pytest.raises(pyjwt.ExpiredSignatureError):
            manager.refresh_access_token(refresh_token)


class TestJWTTokenRevocation:
    """Test JWT token revocation."""

    def test_revoke_token(self):
        """Test revoking token."""
        manager = JWTManager(secret_key="test_secret")

        # Create token with JTI
        token_str = manager.create_access_token("user123")
        payload_dict = pyjwt.decode(token_str, "test_secret", algorithms=["HS256"])
        payload_dict["jti"] = "unique_jti_123"

        # Create new token with JTI
        token_with_jti = pyjwt.encode(payload_dict, "test_secret", algorithm="HS256")

        # Revoke token
        manager.revoke_token(token_with_jti)

        # Verify revoked token fails
        with pytest.raises(ValueError, match="Token has been revoked"):
            manager.verify_token(token_with_jti)

    def test_revoke_invalid_token_ignores(self):
        """Test that revoking invalid token is ignored."""
        manager = JWTManager(secret_key="test_secret")

        # Should not raise error
        manager.revoke_token("invalid.token.here")


class TestJWTUtilityMethods:
    """Test JWT utility methods."""

    def test_decode_token_unsafe(self):
        """Test unsafe token decoding."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_access_token("user123", ["tokens:read"])

        payload = manager.decode_token_unsafe(token)

        assert payload["sub"] == "user123"
        assert payload["scopes"] == ["tokens:read"]
        assert payload["token_type"] == "access"

    def test_decode_expired_token_unsafe(self):
        """Test unsafe decoding of expired token."""
        manager = JWTManager(secret_key="test_secret")
        token = manager.create_access_token(
            "user123",
            expires_delta=timedelta(seconds=-1)
        )

        # Should not raise error (no verification)
        payload = manager.decode_token_unsafe(token)
        assert payload["sub"] == "user123"


class TestJWTConfiguration:
    """Test JWT manager configuration."""

    def test_custom_access_token_expiration(self):
        """Test custom access token expiration config."""
        manager = JWTManager(
            secret_key="test_secret",
            access_token_expire_minutes=60
        )

        token = manager.create_access_token("user123")
        payload = pyjwt.decode(token, "test_secret", algorithms=["HS256"])

        exp_time = datetime.fromtimestamp(payload["exp"])
        iat_time = datetime.fromtimestamp(payload["iat"])
        delta = (exp_time - iat_time).total_seconds()

        # Should be ~60 minutes
        assert 3590 <= delta <= 3610

    def test_custom_refresh_token_expiration(self):
        """Test custom refresh token expiration config."""
        manager = JWTManager(
            secret_key="test_secret",
            refresh_token_expire_days=14
        )

        token = manager.create_refresh_token("user123")
        payload = pyjwt.decode(token, "test_secret", algorithms=["HS256"])

        exp_time = datetime.fromtimestamp(payload["exp"])
        iat_time = datetime.fromtimestamp(payload["iat"])
        delta_days = (exp_time - iat_time).total_seconds() / (24 * 60 * 60)

        # Should be ~14 days
        assert 13.99 <= delta_days <= 14.01

    def test_default_secret_key_from_env(self, monkeypatch):
        """Test default secret key from environment."""
        monkeypatch.setenv("JWT_SECRET_KEY", "env_secret")
        manager = JWTManager()

        assert manager.secret_key == "env_secret"

    def test_fallback_secret_key(self, monkeypatch):
        """Test fallback secret key when env not set."""
        monkeypatch.delenv("JWT_SECRET_KEY", raising=False)
        manager = JWTManager()

        assert manager.secret_key == "CHANGE_THIS_SECRET_KEY_IN_PRODUCTION"
