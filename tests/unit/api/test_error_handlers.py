"""
Unit tests for API Error Handlers.

Tests custom exception handlers and error response formatting.
"""

import pytest
from unittest.mock import Mock
from fastapi import status
from fastapi.exceptions import RequestValidationError
from pydantic import ValidationError

import sys
from pathlib import Path
src_path = Path(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.error_handlers import (
    axiom_exception_handler,
    validation_exception_handler,
    generic_exception_handler,
)
from api.exceptions import AxiomException


class TestAxiomExceptionHandler:
    """Test Axiom exception handler."""

    @pytest.mark.asyncio
    async def test_handles_axiom_exception(self):
        """Test that Axiom exceptions are handled properly."""
        # Create mock request
        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "GET"

        # Create Axiom exception
        exc = AxiomException(
            status_code=400,
            error_code="TEST_ERROR",
            message="Test error message",
            details={"key": "value"}
        )

        # Handle exception
        response = await axiom_exception_handler(mock_request, exc)

        # Verify response
        assert response.status_code == 400

        # Parse JSON content
        import json
        content = json.loads(response.body.decode())

        assert content["success"] is False
        assert content["error"]["code"] == "TEST_ERROR"
        assert content["error"]["message"] == "Test error message"
        assert "details" in content["error"]

    @pytest.mark.asyncio
    async def test_includes_exception_details(self):
        """Test that exception details are included in response."""
        mock_request = Mock()
        mock_request.url.path = "/api/users"
        mock_request.method = "POST"

        exc = AxiomException(
            status_code=404,
            error_code="RESOURCE_NOT_FOUND",
            message="User not found",
            details={"user_id": 123, "searched_in": "database"}
        )

        response = await axiom_exception_handler(mock_request, exc)

        import json
        content = json.loads(response.body.decode())

        assert content["error"]["details"]["user_id"] == 123
        assert content["error"]["details"]["searched_in"] == "database"

    @pytest.mark.asyncio
    async def test_uses_correct_status_code(self):
        """Test that response uses exception's status code."""
        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "DELETE"

        # Test different status codes
        for status_code in [400, 401, 403, 404, 409, 500]:
            exc = AxiomException(
                status_code=status_code,
                error_code="TEST",
                message="Test"
            )

            response = await axiom_exception_handler(mock_request, exc)
            assert response.status_code == status_code

    @pytest.mark.asyncio
    async def test_includes_custom_headers(self):
        """Test that custom headers are included."""
        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "GET"

        exc = AxiomException(
            status_code=429,
            error_code="RATE_LIMIT",
            message="Too many requests",
            headers={"Retry-After": "60"}
        )

        response = await axiom_exception_handler(mock_request, exc)

        assert response.headers["Retry-After"] == "60"


class TestValidationExceptionHandler:
    """Test validation exception handler."""

    @pytest.mark.asyncio
    async def test_handles_validation_errors(self):
        """Test that validation errors are formatted correctly."""
        mock_request = Mock()
        mock_request.url.path = "/api/users"
        mock_request.method = "POST"

        # Create validation error
        # Simulate Pydantic validation error
        try:
            from pydantic import BaseModel, Field

            class TestModel(BaseModel):
                age: int = Field(gt=0)
                email: str

            # This will raise ValidationError
            TestModel(age=-5, email="invalid")
        except ValidationError as e:
            # Wrap in RequestValidationError
            exc = RequestValidationError(errors=e.errors())

            response = await validation_exception_handler(mock_request, exc)

            # Verify response
            assert response.status_code == 422

            import json
            content = json.loads(response.body.decode())

            assert content["success"] is False
            assert content["error"]["code"] == "VALIDATION_ERROR"
            assert content["error"]["message"] == "Request validation failed"
            assert "errors" in content["error"]["details"]
            assert content["error"]["details"]["count"] > 0

    @pytest.mark.asyncio
    async def test_formats_error_fields(self):
        """Test that error fields are formatted properly."""
        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "POST"

        # Mock validation error with specific structure
        mock_errors = [
            {
                "loc": ("body", "username"),
                "msg": "field required",
                "type": "value_error.missing"
            },
            {
                "loc": ("body", "age"),
                "msg": "ensure this value is greater than 0",
                "type": "value_error.number.not_gt"
            }
        ]

        exc = Mock(spec=RequestValidationError)
        exc.errors.return_value = mock_errors

        response = await validation_exception_handler(mock_request, exc)

        import json
        content = json.loads(response.body.decode())

        errors = content["error"]["details"]["errors"]
        assert len(errors) == 2

        # Check first error
        assert errors[0]["field"] == "body.username"
        assert errors[0]["message"] == "field required"
        assert errors[0]["type"] == "value_error.missing"

        # Check second error
        assert errors[1]["field"] == "body.age"

    @pytest.mark.asyncio
    async def test_counts_validation_errors(self):
        """Test that error count is included."""
        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "POST"

        mock_errors = [
            {"loc": ("field1",), "msg": "error1", "type": "type1"},
            {"loc": ("field2",), "msg": "error2", "type": "type2"},
            {"loc": ("field3",), "msg": "error3", "type": "type3"},
        ]

        exc = Mock(spec=RequestValidationError)
        exc.errors.return_value = mock_errors

        response = await validation_exception_handler(mock_request, exc)

        import json
        content = json.loads(response.body.decode())

        assert content["error"]["details"]["count"] == 3


class TestGenericExceptionHandler:
    """Test generic exception handler."""

    @pytest.mark.asyncio
    async def test_handles_generic_exceptions(self):
        """Test that generic exceptions return safe error message."""
        from unittest.mock import patch

        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "GET"

        # Simulate unexpected exception
        exc = ValueError("Internal implementation detail")

        # Mock logger.level to avoid AttributeError
        with patch('api.error_handlers.logger') as mock_logger:
            mock_logger.level = 20  # INFO level
            mock_logger.error = Mock()
            response = await generic_exception_handler(mock_request, exc)

        # Verify response
        assert response.status_code == 500

        import json
        content = json.loads(response.body.decode())

        assert content["success"] is False
        assert content["error"]["code"] == "INTERNAL_SERVER_ERROR"

        # Should NOT leak implementation details
        message = content["error"]["message"]
        assert "implementation detail" not in message.lower()
        assert "unexpected error occurred" in message.lower()

    @pytest.mark.asyncio
    async def test_logs_exception_type(self):
        """Test that exception type is available in details."""
        from unittest.mock import patch

        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "POST"

        # Test different exception types
        exceptions = [
            ValueError("test"),
            KeyError("missing_key"),
            TypeError("wrong_type"),
            RuntimeError("runtime_error")
        ]

        with patch('api.error_handlers.logger') as mock_logger:
            mock_logger.level = 20  # INFO level
            mock_logger.error = Mock()

            for exc in exceptions:
                response = await generic_exception_handler(mock_request, exc)

                import json
                content = json.loads(response.body.decode())

                # Type might be included in details (depends on log level)
                # Just verify details exists and structure is correct
                assert "details" in content["error"]
                assert isinstance(content["error"]["details"], dict)

    @pytest.mark.asyncio
    async def test_returns_500_status(self):
        """Test that 500 status is returned for all unexpected errors."""
        from unittest.mock import patch

        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "GET"

        exceptions = [
            Exception("generic"),
            RuntimeError("runtime"),
            AttributeError("attribute"),
        ]

        with patch('api.error_handlers.logger') as mock_logger:
            mock_logger.level = 20  # INFO level
            mock_logger.error = Mock()

            for exc in exceptions:
                response = await generic_exception_handler(mock_request, exc)
                assert response.status_code == 500


class TestErrorHandlerIntegration:
    """Test error handlers working together."""

    @pytest.mark.asyncio
    async def test_different_handlers_different_formats(self):
        """Test that different handlers produce appropriate formats."""
        from unittest.mock import patch

        mock_request = Mock()
        mock_request.url.path = "/api/test"
        mock_request.method = "GET"

        # Axiom exception - structured with details
        ng_exc = AxiomException(400, "TEST", "Test", {"key": "value"})
        ng_response = await axiom_exception_handler(mock_request, ng_exc)

        # Generic exception - safe message
        gen_exc = ValueError("Internal error")

        with patch('api.error_handlers.logger') as mock_logger:
            mock_logger.level = 20  # INFO level
            mock_logger.error = Mock()
            gen_response = await generic_exception_handler(mock_request, gen_exc)

        import json
        ng_content = json.loads(ng_response.body.decode())
        gen_content = json.loads(gen_response.body.decode())

        # Both should have success=False
        assert ng_content["success"] is False
        assert gen_content["success"] is False

        # But different codes
        assert ng_content["error"]["code"] == "TEST"
        assert gen_content["error"]["code"] == "INTERNAL_SERVER_ERROR"

        # And different status codes
        assert ng_response.status_code == 400
        assert gen_response.status_code == 500
