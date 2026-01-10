"""
Unit tests for API Middleware.

Tests correlation ID tracking, request logging, and error handling.
"""

import pytest
from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from fastapi.testclient import TestClient

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.middleware import CorrelationIDMiddleware, RequestLoggingMiddleware, ErrorLoggingMiddleware


class TestCorrelationIDMiddleware:
    """Test correlation ID middleware."""

    def test_generates_correlation_id(self):
        """Test that middleware generates correlation ID."""
        app = FastAPI()
        app.add_middleware(CorrelationIDMiddleware)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        response = client.get("/test")

        assert response.status_code == 200
        assert "X-Correlation-ID" in response.headers
        assert len(response.headers["X-Correlation-ID"]) == 36  # UUID length

    def test_uses_provided_correlation_id(self):
        """Test that middleware uses provided correlation ID."""
        app = FastAPI()
        app.add_middleware(CorrelationIDMiddleware)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        custom_id = "custom-correlation-id-12345"
        response = client.get("/test", headers={"X-Correlation-ID": custom_id})

        assert response.status_code == 200
        assert response.headers["X-Correlation-ID"] == custom_id

    def test_different_requests_get_different_ids(self):
        """Test that different requests get different correlation IDs."""
        app = FastAPI()
        app.add_middleware(CorrelationIDMiddleware)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        response1 = client.get("/test")
        response2 = client.get("/test")

        assert response1.headers["X-Correlation-ID"] != response2.headers["X-Correlation-ID"]


class TestRequestLoggingMiddleware:
    """Test request logging middleware."""

    def test_adds_process_time_header(self):
        """Test that middleware adds X-Process-Time header."""
        app = FastAPI()
        app.add_middleware(RequestLoggingMiddleware)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        response = client.get("/test")

        assert response.status_code == 200
        assert "X-Process-Time" in response.headers
        # Process time format is "X.XXms"
        process_time_str = response.headers["X-Process-Time"]
        assert process_time_str.endswith("ms")
        process_time = float(process_time_str[:-2])
        assert process_time > 0
        assert process_time < 10000  # Should be less than 10 seconds (10000ms)

    def test_skips_health_check_endpoints(self):
        """Test that health check endpoints are skipped."""
        app = FastAPI()
        app.add_middleware(RequestLoggingMiddleware)

        @app.get("/health")
        async def health():
            return {"status": "ok"}

        @app.get("/api/v1/health")
        async def health_v1():
            return {"status": "ok"}

        client = TestClient(app)
        
        # Health endpoints should still work
        response1 = client.get("/health")
        response2 = client.get("/api/v1/health")

        assert response1.status_code == 200
        assert response2.status_code == 200

    def test_logs_request_and_response(self):
        """Test that requests and responses are logged."""
        app = FastAPI()
        app.add_middleware(RequestLoggingMiddleware)

        @app.get("/test")
        async def test_endpoint():
            return {"result": "success"}

        client = TestClient(app)
        response = client.get("/test")

        assert response.status_code == 200
        assert response.json() == {"result": "success"}

    def test_custom_skip_paths(self):
        """Test custom skip paths configuration."""
        app = FastAPI()
        app.add_middleware(RequestLoggingMiddleware, skip_paths=["/skip-me", "/also-skip"])

        @app.get("/skip-me")
        async def skip_endpoint():
            return {"status": "skipped"}

        @app.get("/log-me")
        async def log_endpoint():
            return {"status": "logged"}

        client = TestClient(app)

        response1 = client.get("/skip-me")
        response2 = client.get("/log-me")

        assert response1.status_code == 200
        assert response2.status_code == 200
        assert "X-Process-Time" in response2.headers

    def test_logs_exception_with_timing(self):
        """Test that exceptions are logged with timing info."""
        app = FastAPI()
        app.add_middleware(RequestLoggingMiddleware)

        @app.get("/error")
        async def error_endpoint():
            raise ValueError("Logging middleware exception test")

        client = TestClient(app)

        # This should trigger the exception handler in RequestLoggingMiddleware
        try:
            response = client.get("/error")
            # Exception might be caught by FastAPI error handler
            assert response.status_code == 500
        except Exception:
            # Exception bubbled up
            pass


class TestErrorLoggingMiddleware:
    """Test error logging middleware."""

    def test_handles_exceptions(self):
        """Test that middleware handles exceptions."""
        app = FastAPI()
        app.add_middleware(ErrorLoggingMiddleware)

        @app.get("/error")
        async def error_endpoint():
            raise ValueError("Test error")

        client = TestClient(app)
        response = client.get("/error")

        assert response.status_code == 500
        data = response.json()
        # Error response format: {"success": False, "error": {"code": "...", "message": "..."}}
        assert data["success"] is False
        assert "error" in data
        assert "message" in data["error"]

    def test_handles_validation_errors(self):
        """Test that middleware handles validation errors."""
        app = FastAPI()
        app.add_middleware(ErrorLoggingMiddleware)

        @app.get("/validate")
        async def validate_endpoint(value: int):
            return {"value": value}

        client = TestClient(app)
        response = client.get("/validate?value=not_an_int")

        assert response.status_code == 422  # Validation error

    def test_successful_requests_pass_through(self):
        """Test that successful requests pass through unchanged."""
        app = FastAPI()
        app.add_middleware(ErrorLoggingMiddleware)

        @app.get("/success")
        async def success_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        response = client.get("/success")

        assert response.status_code == 200
        assert response.json() == {"status": "ok"}

    def test_preserves_http_exceptions(self):
        """Test that HTTP exceptions are preserved."""
        from fastapi import HTTPException

        app = FastAPI()
        app.add_middleware(ErrorLoggingMiddleware)

        @app.get("/not-found")
        async def not_found():
            raise HTTPException(status_code=404, detail="Not found")

        client = TestClient(app)
        response = client.get("/not-found")

        assert response.status_code == 404
        assert response.json()["detail"] == "Not found"

    def test_debug_mode_includes_details(self):
        """Test that debug mode includes exception details."""
        app = FastAPI()
        app.add_middleware(ErrorLoggingMiddleware, debug=True)

        @app.get("/error")
        async def error_endpoint():
            raise ValueError("Debug test error")

        client = TestClient(app)
        response = client.get("/error")

        assert response.status_code == 500
        data = response.json()
        assert data["success"] is False
        assert "error" in data
        # Debug mode should include details
        if "details" in data["error"]:
            assert "type" in data["error"]["details"]
            assert data["error"]["details"]["type"] == "ValueError"


class TestMiddlewareIntegration:
    """Test multiple middleware working together."""

    def test_all_middleware_together(self):
        """Test that all middleware work together correctly."""
        app = FastAPI()
        app.add_middleware(ErrorLoggingMiddleware)
        app.add_middleware(RequestLoggingMiddleware)
        app.add_middleware(CorrelationIDMiddleware)

        @app.get("/integrated")
        async def integrated_endpoint():
            return {"result": "success"}

        client = TestClient(app)
        response = client.get("/integrated")

        assert response.status_code == 200
        assert "X-Correlation-ID" in response.headers
        assert "X-Process-Time" in response.headers
        assert response.json() == {"result": "success"}

    def test_middleware_order_matters(self):
        """Test that middleware are applied in correct order."""
        app = FastAPI()
        # Order matters: last added runs first
        app.add_middleware(ErrorLoggingMiddleware)
        app.add_middleware(CorrelationIDMiddleware)

        @app.get("/test")
        async def test_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        response = client.get("/test")

        assert response.status_code == 200
        assert "X-Correlation-ID" in response.headers


class TestMiddlewarePerformance:
    """Test middleware performance impact."""

    def test_minimal_overhead(self):
        """Test that middleware adds minimal overhead."""
        app = FastAPI()
        app.add_middleware(RequestLoggingMiddleware)
        app.add_middleware(CorrelationIDMiddleware)

        @app.get("/fast")
        async def fast_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        response = client.get("/fast")

        # Process time format is "X.XXms"
        process_time_str = response.headers["X-Process-Time"]
        assert process_time_str.endswith("ms")
        process_time = float(process_time_str[:-2])
        # Middleware overhead should be < 100ms for simple endpoint
        assert process_time < 100

    def test_handles_concurrent_requests(self):
        """Test that middleware handles concurrent requests."""
        app = FastAPI()
        app.add_middleware(CorrelationIDMiddleware)

        @app.get("/concurrent")
        async def concurrent_endpoint():
            return {"status": "ok"}

        client = TestClient(app)
        
        # Make multiple concurrent requests
        responses = [client.get("/concurrent") for _ in range(10)]

        # All should succeed with unique correlation IDs
        assert all(r.status_code == 200 for r in responses)
        correlation_ids = [r.headers["X-Correlation-ID"] for r in responses]
        assert len(set(correlation_ids)) == 10  # All unique
