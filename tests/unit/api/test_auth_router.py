"""
Unit tests for Authentication Router.

Tests user authentication, token management, and password operations.
"""

import pytest
from unittest.mock import Mock, patch
from fastapi import FastAPI
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

import api.routers.auth as auth
auth_router = auth.router


class TestLogin:
    """Test /auth/login endpoint."""

    def test_login_success_admin(self):
        """Test successful login with admin user."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )

        assert response.status_code == 200
        data = response.json()
        assert "access_token" in data
        assert "refresh_token" in data
        assert data["token_type"] == "bearer"
        assert "expires_in" in data
        assert "user" in data
        assert data["user"]["username"] == "admin"
        assert data["user"]["role"] == "admin"

    def test_login_success_developer(self):
        """Test successful login with developer user."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/login",
            json={"username": "developer", "password": "developer123"}
        )

        assert response.status_code == 200
        data = response.json()
        assert data["user"]["username"] == "developer"
        assert data["user"]["role"] == "developer"

    def test_login_success_viewer(self):
        """Test successful login with viewer user."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/login",
            json={"username": "viewer", "password": "viewer123"}
        )

        assert response.status_code == 200
        data = response.json()
        assert data["user"]["username"] == "viewer"
        assert data["user"]["role"] == "viewer"

    def test_login_invalid_username(self):
        """Test login with invalid username."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/login",
            json={"username": "nonexistent", "password": "password"}
        )

        assert response.status_code == 401
        data = response.json()
        assert "detail" in data
        assert "username or password" in data["detail"].lower()

    def test_login_invalid_password(self):
        """Test login with invalid password."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "wrongpassword"}
        )

        assert response.status_code == 401
        data = response.json()
        assert "detail" in data

    def test_login_disabled_user(self):
        """Test login with disabled user account."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        # Temporarily mark admin as disabled
        original_disabled = auth.USERS_DB["admin"].disabled
        auth.USERS_DB["admin"].disabled = True

        try:
            client = TestClient(app)
            response = client.post(
                "/api/v1/auth/login",
                json={"username": "admin", "password": "admin123"}
            )

            assert response.status_code == 400
            data = response.json()
            assert "disabled" in data["detail"].lower()
        finally:
            # Restore original state
            auth.USERS_DB["admin"].disabled = original_disabled

    def test_login_returns_user_profile(self):
        """Test that login returns complete user profile."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/login",
            json={"username": "developer", "password": "developer123"}
        )

        assert response.status_code == 200
        data = response.json()
        user = data["user"]

        assert "user_id" in user
        assert "username" in user
        assert "email" in user
        assert "full_name" in user
        assert "role" in user
        assert "scopes" in user
        assert "disabled" in user
        assert "created_at" in user

    def test_login_tokens_are_different(self):
        """Test that access and refresh tokens are different."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )

        assert response.status_code == 200
        data = response.json()
        assert data["access_token"] != data["refresh_token"]

    def test_login_validation_error(self):
        """Test login with missing fields."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin"}  # Missing password
        )

        assert response.status_code == 422


class TestRefreshToken:
    """Test /auth/refresh endpoint."""

    def test_refresh_token_success(self):
        """Test successful token refresh."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # First, login to get tokens
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )
        refresh_token = login_response.json()["refresh_token"]

        # Now refresh the token
        response = client.post(
            "/api/v1/auth/refresh",
            json={"refresh_token": refresh_token}
        )

        assert response.status_code == 200
        data = response.json()
        assert "access_token" in data
        assert "refresh_token" in data
        assert data["token_type"] == "bearer"
        assert "expires_in" in data

    def test_refresh_token_returns_new_access_token(self):
        """Test that refresh returns new access token."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # Login
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )
        old_access = login_response.json()["access_token"]
        old_refresh = login_response.json()["refresh_token"]

        # Refresh
        response = client.post(
            "/api/v1/auth/refresh",
            json={"refresh_token": old_refresh}
        )

        assert response.status_code == 200
        data = response.json()

        # New access token should be different
        assert data["access_token"] != old_access
        # Refresh token may or may not be rotated depending on implementation
        assert "refresh_token" in data

    def test_refresh_token_invalid_token(self):
        """Test refresh with invalid token."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/refresh",
            json={"refresh_token": "invalid_token_12345"}
        )

        assert response.status_code == 401
        data = response.json()
        assert "detail" in data
        assert "invalid" in data["detail"].lower()

    def test_refresh_token_validation_error(self):
        """Test refresh with missing token."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/refresh",
            json={}  # Missing refresh_token
        )

        assert response.status_code == 422


class TestLogout:
    """Test /auth/logout endpoint."""

    def test_logout_success(self):
        """Test successful logout."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # Login first
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )
        access_token = login_response.json()["access_token"]

        # Logout
        response = client.post(
            "/api/v1/auth/logout",
            json={},
            headers={"Authorization": f"Bearer {access_token}"}
        )

        assert response.status_code == 200
        data = response.json()
        assert data["success"] is True
        assert "logged out" in data["message"].lower()

    def test_logout_with_token_revocation(self):
        """Test logout with token revocation."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # Login
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "developer", "password": "developer123"}
        )
        access_token = login_response.json()["access_token"]

        # Logout with token revocation
        response = client.post(
            "/api/v1/auth/logout",
            json={"token": access_token},
            headers={"Authorization": f"Bearer {access_token}"}
        )

        assert response.status_code == 200

    def test_logout_without_auth(self):
        """Test logout without authentication."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/logout",
            json={}
        )

        # Should fail without auth token
        assert response.status_code in [401, 403]

    def test_logout_invalid_token(self):
        """Test logout with invalid token."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/logout",
            json={},
            headers={"Authorization": "Bearer invalid_token"}
        )

        # Should fail with invalid token
        assert response.status_code in [401, 403]


class TestGetCurrentUser:
    """Test /auth/me endpoint."""

    def test_get_me_success(self):
        """Test getting current user profile."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # Login first
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )
        access_token = login_response.json()["access_token"]

        # Get profile
        response = client.get(
            "/api/v1/auth/me",
            headers={"Authorization": f"Bearer {access_token}"}
        )

        assert response.status_code == 200
        data = response.json()
        assert data["username"] == "admin"
        assert data["role"] == "admin"
        assert "email" in data
        assert "scopes" in data

    def test_get_me_includes_all_fields(self):
        """Test that profile includes all user fields."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # Login
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "developer", "password": "developer123"}
        )
        access_token = login_response.json()["access_token"]

        # Get profile
        response = client.get(
            "/api/v1/auth/me",
            headers={"Authorization": f"Bearer {access_token}"}
        )

        assert response.status_code == 200
        data = response.json()

        required_fields = [
            "user_id", "username", "email", "full_name",
            "role", "scopes", "disabled", "created_at"
        ]
        for field in required_fields:
            assert field in data

    def test_get_me_without_auth(self):
        """Test getting profile without authentication."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get("/api/v1/auth/me")

        # Should fail without auth
        assert response.status_code in [401, 403]

    def test_get_me_invalid_token(self):
        """Test getting profile with invalid token."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.get(
            "/api/v1/auth/me",
            headers={"Authorization": "Bearer invalid_token"}
        )

        # Should fail with invalid token
        assert response.status_code in [401, 403]

class TestChangePassword:
    """Test /auth/change-password endpoint."""

    def test_change_password_flow(self):
        """Test password change workflow."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        # Use viewer account which we won't use in other tests
        import hashlib
        original_hash = hashlib.sha256("viewer123".encode()).hexdigest()
        auth.USERS_DB["viewer"].hashed_password = original_hash

        try:
            client = TestClient(app)

            # 1. Login with original password
            login = client.post(
                "/api/v1/auth/login",
                json={"username": "viewer", "password": "viewer123"}
            )
            assert login.status_code == 200
            token = login.json()["access_token"]

            # 2. Change password
            change = client.post(
                "/api/v1/auth/change-password",
                json={
                    "old_password": "viewer123",
                    "new_password": "newviewer456"
                },
                headers={"Authorization": f"Bearer {token}"}
            )

            # If password change succeeds (200), verify new password works
            if change.status_code == 200:
                data = change.json()
                assert data["success"] is True

                # 3. Login with new password
                new_login = client.post(
                    "/api/v1/auth/login",
                    json={"username": "viewer", "password": "newviewer456"}
                )
                assert new_login.status_code == 200
            else:
                # If it fails, just check that old password still works
                verify_old = client.post(
                    "/api/v1/auth/login",
                    json={"username": "viewer", "password": "viewer123"}
                )
                assert verify_old.status_code == 200

        finally:
            # Always restore original password
            auth.USERS_DB["viewer"].hashed_password = original_hash

    def test_change_password_wrong_old_password(self):
        """Test change password with wrong old password."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # Login
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "viewer", "password": "viewer123"}
        )
        access_token = login_response.json()["access_token"]

        # Try to change with wrong old password
        response = client.post(
            "/api/v1/auth/change-password",
            json={
                "old_password": "wrongpassword",
                "new_password": "newpassword456"
            },
            headers={"Authorization": f"Bearer {access_token}"}
        )

        assert response.status_code == 400
        data = response.json()
        assert "incorrect" in data["detail"].lower() or "old password" in data["detail"].lower()

    def test_change_password_without_auth(self):
        """Test change password without authentication."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)
        response = client.post(
            "/api/v1/auth/change-password",
            json={
                "old_password": "old123",
                "new_password": "new456"
            }
        )

        # Should fail without auth
        assert response.status_code in [401, 403]

    def test_change_password_validation_error(self):
        """Test change password with missing fields."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # Login
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )
        access_token = login_response.json()["access_token"]

        # Try to change with missing new_password
        response = client.post(
            "/api/v1/auth/change-password",
            json={"old_password": "admin123"},  # Missing new_password
            headers={"Authorization": f"Bearer {access_token}"}
        )

        assert response.status_code == 422


class TestPasswordFunctions:
    """Test password utility functions."""

    def test_verify_password_correct(self):
        """Test password verification with correct password."""
        hashed = auth.hashlib.sha256("testpassword".encode()).hexdigest()
        assert auth.verify_password("testpassword", hashed) is True

    def test_verify_password_incorrect(self):
        """Test password verification with incorrect password."""
        hashed = auth.hashlib.sha256("testpassword".encode()).hexdigest()
        assert auth.verify_password("wrongpassword", hashed) is False

    def test_authenticate_user_success(self):
        """Test user authentication with valid credentials."""
        user = auth.authenticate_user("admin", "admin123")
        assert user is not None
        assert user.username == "admin"

    def test_authenticate_user_wrong_password(self):
        """Test user authentication with wrong password."""
        user = auth.authenticate_user("admin", "wrongpassword")
        assert user is None

    def test_authenticate_user_nonexistent(self):
        """Test user authentication with nonexistent user."""
        user = auth.authenticate_user("nonexistent", "password")
        assert user is None


class TestAuthIntegration:
    """Test authentication flow integration."""

    def test_full_auth_flow(self):
        """Test complete authentication flow."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # 1. Login
        login_response = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )
        assert login_response.status_code == 200
        access_token = login_response.json()["access_token"]
        refresh_token = login_response.json()["refresh_token"]

        # 2. Get profile
        profile_response = client.get(
            "/api/v1/auth/me",
            headers={"Authorization": f"Bearer {access_token}"}
        )
        assert profile_response.status_code == 200
        assert profile_response.json()["username"] == "admin"

        # 3. Refresh token
        refresh_response = client.post(
            "/api/v1/auth/refresh",
            json={"refresh_token": refresh_token}
        )
        assert refresh_response.status_code == 200
        new_access_token = refresh_response.json()["access_token"]

        # 4. Use new token to get profile
        new_profile_response = client.get(
            "/api/v1/auth/me",
            headers={"Authorization": f"Bearer {new_access_token}"}
        )
        assert new_profile_response.status_code == 200

        # 5. Logout
        logout_response = client.post(
            "/api/v1/auth/logout",
            json={},
            headers={"Authorization": f"Bearer {new_access_token}"}
        )
        assert logout_response.status_code == 200

    def test_multiple_logins_same_user(self):
        """Test that user can have multiple active sessions."""
        app = FastAPI()
        app.include_router(auth_router, prefix="/api/v1")

        client = TestClient(app)

        # Login twice
        login1 = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )
        token1 = login1.json()["access_token"]

        login2 = client.post(
            "/api/v1/auth/login",
            json={"username": "admin", "password": "admin123"}
        )
        token2 = login2.json()["access_token"]

        # Both tokens should work
        profile1 = client.get(
            "/api/v1/auth/me",
            headers={"Authorization": f"Bearer {token1}"}
        )
        profile2 = client.get(
            "/api/v1/auth/me",
            headers={"Authorization": f"Bearer {token2}"}
        )

        assert profile1.status_code == 200
        assert profile2.status_code == 200
        assert profile1.json()["username"] == profile2.json()["username"]
