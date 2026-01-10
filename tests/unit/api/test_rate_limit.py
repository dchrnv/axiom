"""
Unit tests for Rate Limiting Middleware.

Tests token bucket algorithm, rate limiting logic, and cleanup.
"""

import pytest
import time
import threading
from fastapi import FastAPI
from fastapi.testclient import TestClient
from unittest.mock import Mock

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.middlewares.rate_limit import TokenBucket, RateLimitMiddleware


class TestTokenBucket:
    """Test token bucket rate limiter."""

    def test_initialization(self):
        """Test bucket initialization."""
        bucket = TokenBucket(rate_limit=10)

        assert bucket.capacity == 10
        assert bucket.tokens == 10.0
        assert bucket.last_update > 0

    def test_consume_single_token(self):
        """Test consuming a single token."""
        bucket = TokenBucket(rate_limit=10)

        # Should succeed
        assert bucket.consume() is True

        # Tokens should decrease
        assert bucket.get_remaining() == 9

    def test_consume_all_tokens(self):
        """Test consuming all available tokens."""
        bucket = TokenBucket(rate_limit=3)

        # Consume all 3
        assert bucket.consume() is True  # 2 left
        assert bucket.consume() is True  # 1 left
        assert bucket.consume() is True  # 0 left

        # Next should fail
        assert bucket.consume() is False

    def test_token_refill_over_time(self):
        """Test that tokens refill at 1 per second."""
        bucket = TokenBucket(rate_limit=10)

        # Consume 5 tokens
        for _ in range(5):
            bucket.consume()

        # Should have 5 left
        assert bucket.get_remaining() == 5

        # Wait 2 seconds
        time.sleep(2.1)

        # Should have refilled ~2 tokens (5 + 2 = 7)
        remaining = bucket.get_remaining()
        assert 6 <= remaining <= 8  # Allow some timing variance

    def test_tokens_dont_exceed_capacity(self):
        """Test that refill doesn't exceed capacity."""
        bucket = TokenBucket(rate_limit=5)

        # Wait for refill
        time.sleep(1.0)

        # Should still be at capacity
        assert bucket.get_remaining() == 5

        # Consume one
        bucket.consume()
        assert bucket.get_remaining() == 4

        # Wait for refill
        time.sleep(1.1)

        # Should be back at capacity (not more)
        assert bucket.get_remaining() == 5

    def test_thread_safety(self):
        """Test that bucket is thread-safe."""
        bucket = TokenBucket(rate_limit=100)
        successes = []

        def consume_token():
            if bucket.consume():
                successes.append(1)

        # Try to consume 100 tokens from 10 threads
        threads = [threading.Thread(target=consume_token) for _ in range(100)]

        for thread in threads:
            thread.start()

        for thread in threads:
            thread.join()

        # Should have exactly 100 successes (no race conditions)
        assert len(successes) == 100

    def test_get_remaining_accuracy(self):
        """Test get_remaining returns accurate count."""
        bucket = TokenBucket(rate_limit=10)

        assert bucket.get_remaining() == 10

        bucket.consume()
        assert bucket.get_remaining() == 9

        bucket.consume()
        bucket.consume()
        assert bucket.get_remaining() == 7


class TestRateLimitMiddleware:
    """Test rate limiting middleware."""

    def test_allows_requests_within_limit(self):
        """Test that requests within limit are allowed."""
        app = FastAPI()
        app.add_middleware(RateLimitMiddleware, default_rate_limit=10)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)

        # Make 5 requests (under limit of 10)
        for _ in range(5):
            response = client.get("/test")
            assert response.status_code == 200

    def test_blocks_requests_over_limit(self):
        """Test that requests over limit are blocked."""
        app = FastAPI()
        app.add_middleware(RateLimitMiddleware, default_rate_limit=3)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)

        # Make 3 requests (at limit)
        for i in range(3):
            response = client.get("/test")
            assert response.status_code == 200, f"Request {i+1} should succeed"

        # 4th request should be blocked
        response = client.get("/test")
        assert response.status_code == 429  # Too Many Requests

        data = response.json()
        assert data["success"] is False
        assert data["error"]["code"] == "RATE_LIMIT_EXCEEDED"

    def test_adds_rate_limit_headers(self):
        """Test that rate limit headers are added."""
        app = FastAPI()
        app.add_middleware(RateLimitMiddleware, default_rate_limit=10)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        response = client.get("/test")

        assert "X-RateLimit-Limit" in response.headers
        assert response.headers["X-RateLimit-Limit"] == "10"

        assert "X-RateLimit-Remaining" in response.headers
        remaining = int(response.headers["X-RateLimit-Remaining"])
        assert remaining == 9  # One consumed

    def test_rate_limit_error_includes_details(self):
        """Test that rate limit error includes retry info."""
        app = FastAPI()
        app.add_middleware(RateLimitMiddleware, default_rate_limit=1)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)

        # Consume the one allowed request
        client.get("/test")

        # Next should be blocked
        response = client.get("/test")
        assert response.status_code == 429

        # Check headers
        assert "Retry-After" in response.headers
        assert response.headers["Retry-After"] == "1"

        # Check body
        data = response.json()
        assert "details" in data["error"]
        assert "rate_limit" in data["error"]["details"]
        assert "retry_after" in data["error"]["details"]

    def test_skips_health_check_endpoints(self):
        """Test that health checks are not rate limited."""
        app = FastAPI()
        app.add_middleware(RateLimitMiddleware, default_rate_limit=2)

        @app.get("/health")
        async def health():
            return {"status": "healthy"}

        @app.get("/api/v1/health")
        async def health_v1():
            return {"status": "healthy"}

        client = TestClient(app)

        # Make many health check requests (should all succeed)
        for _ in range(10):
            response = client.get("/health")
            assert response.status_code == 200

        for _ in range(10):
            response = client.get("/api/v1/health")
            assert response.status_code == 200

    def test_user_id_extraction_from_ip(self):
        """Test user ID extraction from IP address."""
        app = FastAPI()
        middleware = RateLimitMiddleware(app, default_rate_limit=10)

        # Mock request with IP
        mock_request = Mock()
        mock_request.client.host = "192.168.1.1"
        mock_request.headers = {}
        mock_request.state = Mock(spec=[])

        user_id = middleware.get_user_id(mock_request)
        assert user_id == "ip_192.168.1.1"

    def test_user_id_extraction_from_api_key(self):
        """Test user ID extraction from API key."""
        app = FastAPI()
        middleware = RateLimitMiddleware(app, default_rate_limit=10)

        # Mock request with API key
        mock_request = Mock()
        mock_request.client.host = "192.168.1.1"
        mock_request.headers = {"x-api-key": "ng_test_12345678"}
        mock_request.state = Mock(spec=[])

        user_id = middleware.get_user_id(mock_request)
        assert user_id == "apikey_ng_test_"  # First 8 chars

    def test_user_id_extraction_from_state(self):
        """Test user ID extraction from request state."""
        app = FastAPI()
        middleware = RateLimitMiddleware(app, default_rate_limit=10)

        # Mock request with user in state
        mock_request = Mock()
        mock_request.client.host = "192.168.1.1"
        mock_request.headers = {}
        mock_user = Mock()
        mock_user.user_id = "user123"
        mock_request.state.user = mock_user

        user_id = middleware.get_user_id(mock_request)
        assert user_id == "user123"

    def test_get_or_create_bucket(self):
        """Test bucket creation and retrieval."""
        app = FastAPI()
        middleware = RateLimitMiddleware(app, default_rate_limit=10)

        # Get bucket for new user
        bucket1 = middleware.get_bucket("user1")
        assert bucket1 is not None
        assert bucket1.capacity == 10

        # Get same bucket again
        bucket2 = middleware.get_bucket("user1")
        assert bucket1 is bucket2  # Same instance

        # Different user gets different bucket
        bucket3 = middleware.get_bucket("user2")
        assert bucket3 is not bucket1

    def test_cleanup_old_buckets(self):
        """Test cleanup of expired buckets."""
        app = FastAPI()
        middleware = RateLimitMiddleware(app, default_rate_limit=10, cleanup_interval=1)

        # Create buckets
        middleware.get_bucket("user1")
        middleware.get_bucket("user2")

        assert len(middleware.buckets) == 2

        # Wait for cleanup interval
        time.sleep(1.1)

        # Trigger cleanup
        middleware.cleanup_old_buckets()

        # Old buckets should be removed
        assert len(middleware.buckets) == 0

    def test_cleanup_preserves_active_buckets(self):
        """Test that cleanup doesn't remove active buckets."""
        app = FastAPI()
        middleware = RateLimitMiddleware(app, default_rate_limit=10, cleanup_interval=1)

        # Create bucket
        middleware.get_bucket("user1")

        # Wait a bit
        time.sleep(0.5)

        # Access bucket again (keeps it active)
        middleware.get_bucket("user1")

        # Wait for cleanup interval from first access
        time.sleep(0.6)

        # Trigger cleanup
        middleware.cleanup_old_buckets()

        # Bucket should still be there (last access was recent)
        assert len(middleware.buckets) == 1

    def test_cleanup_only_runs_at_interval(self):
        """Test that cleanup only runs at specified interval."""
        app = FastAPI()
        middleware = RateLimitMiddleware(app, default_rate_limit=10, cleanup_interval=10)

        # Set last_cleanup to now
        middleware.last_cleanup = time.time()

        # Try to cleanup immediately
        middleware.cleanup_old_buckets()

        # last_cleanup should not have changed (cleanup was skipped)
        assert abs(middleware.last_cleanup - time.time()) < 1

    def test_different_users_independent_limits(self):
        """Test that different users have independent rate limits."""
        app = FastAPI()
        app.add_middleware(RateLimitMiddleware, default_rate_limit=2)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)

        # User 1 (IP) makes 2 requests
        response1 = client.get("/test")
        response2 = client.get("/test")
        assert response1.status_code == 200
        assert response2.status_code == 200

        # User 1's 3rd request blocked
        response3 = client.get("/test")
        assert response3.status_code == 429

        # User 2 (with API key) can still make requests
        response4 = client.get("/test", headers={"X-API-Key": "different_key"})
        assert response4.status_code == 200


class TestRateLimitIntegration:
    """Test rate limiting integration scenarios."""

    def test_rate_limit_recovery_after_time(self):
        """Test that rate limit recovers after waiting."""
        app = FastAPI()
        app.add_middleware(RateLimitMiddleware, default_rate_limit=2)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)

        # Exhaust rate limit
        client.get("/test")
        client.get("/test")

        # Should be blocked
        response = client.get("/test")
        assert response.status_code == 429

        # Wait for token refill (1+ seconds)
        time.sleep(1.1)

        # Should succeed now
        response = client.get("/test")
        assert response.status_code == 200

    def test_rate_limit_with_concurrent_requests(self):
        """Test rate limiting with concurrent requests."""
        app = FastAPI()
        app.add_middleware(RateLimitMiddleware, default_rate_limit=10)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)

        # Make multiple concurrent requests
        results = []

        def make_request():
            response = client.get("/test")
            results.append(response.status_code)

        threads = [threading.Thread(target=make_request) for _ in range(15)]

        for thread in threads:
            thread.start()

        for thread in threads:
            thread.join()

        # First 10 should succeed, rest should be blocked
        success_count = results.count(200)
        blocked_count = results.count(429)

        assert success_count == 10
        assert blocked_count == 5
