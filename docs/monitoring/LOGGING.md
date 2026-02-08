# NeuroGraph - Structured Logging Guide

**Version:** v0.67.0
**Last Updated:** 2026-01-10

## Overview

NeuroGraph использует structured JSON logging для улучшенной отладки, мониторинга и audit trail. Все логи включают correlation IDs, timing metrics, и контекстную информацию.

## Features

✅ **JSON-formatted logs** - Легко парсятся Elasticsearch, Loki, CloudWatch
✅ **Correlation ID tracking** - Отслеживание запросов через все компоненты
✅ **Request/Response logging** - Автоматическое логирование HTTP операций
✅ **Error tracking** - Детальные stack traces и exception info
✅ **Performance metrics** - Request duration и latency tracking
✅ **Context-aware** - Async-safe context variables для threading

## Log Format

### JSON Structure

```json
{
  "timestamp": "2026-01-10T15:30:45.123456+00:00",
  "level": "INFO",
  "logger": "api.routers.tokens",
  "message": "Token created successfully",
  "correlation_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "service": "api",
  "component": "tokens",
  "token_id": 12345,
  "user_id": "user123",
  "duration_ms": 42.5
}
```

### Fields

| Field | Type | Description |
|-------|------|-------------|
| `timestamp` | string (ISO 8601) | UTC timestamp |
| `level` | string | Log level (DEBUG, INFO, WARNING, ERROR, CRITICAL) |
| `logger` | string | Logger name (module path) |
| `message` | string | Human-readable message |
| `correlation_id` | string (UUID) | Request correlation ID |
| `service` | string | Service name (e.g., "api", "worker") |
| `component` | string | Component name (e.g., "tokens", "grid") |
| `function` | string | Function name (for WARNING+) |
| `line` | int | Line number (for WARNING+) |
| `file` | string | File path (for WARNING+) |
| `exception` | object | Exception details (if present) |

## Configuration

### Environment Variables

```bash
# Log level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
LOG_LEVEL=INFO

# Enable JSON formatting (true/false)
LOG_JSON_FORMAT=true

# Enable correlation ID tracking (true/false)
LOG_CORRELATION_TRACKING=true

# Log request/response bodies (false in production)
LOG_REQUEST_BODY=false
LOG_RESPONSE_BODY=false
```

### Programmatic Configuration

```python
from api.logging_config import setup_logging, get_logger

# Setup logging
setup_logging(
    level="DEBUG",
    json_format=True,
    correlation_tracking=True
)

# Get logger with context
logger = get_logger(__name__, service="api", component="tokens")
```

## Usage Examples

### Basic Logging

```python
from api.logging_config import get_logger

logger = get_logger(__name__, component="tokens")

# Simple log
logger.info("Token created")

# Log with extra context
logger.info(
    "Token created successfully",
    extra={
        "token_id": 12345,
        "user_id": "user123",
        "duration_ms": 42.5
    }
)
```

### Logging Critical Operations

```python
# Token creation
logger.info(
    "Creating token",
    extra={
        "user_id": user_id,
        "token_type": "access",
        "expires_in": 3600
    }
)

# Database operations
logger.debug(
    "Executing database query",
    extra={
        "query_type": "select",
        "table": "tokens",
        "filters": filters
    }
)

# Authentication
logger.warning(
    "Failed authentication attempt",
    extra={
        "user_id": user_id,
        "ip_address": request.client.host,
        "reason": "invalid_credentials"
    }
)
```

### Error Logging

```python
try:
    result = dangerous_operation()
except Exception as e:
    logger.error(
        "Operation failed",
        extra={
            "operation": "dangerous_operation",
            "error_type": type(e).__name__
        },
        exc_info=True  # Include stack trace
    )
    raise
```

### Correlation ID Tracking

```python
from api.logging_config import set_correlation_id, get_correlation_id

# Set correlation ID (automatically done by middleware)
set_correlation_id("a1b2c3d4-e5f6-7890-abcd-ef1234567890")

# Get current correlation ID
correlation_id = get_correlation_id()
logger.info(f"Processing request {correlation_id}")
```

## Automatic Request Logging

### Middleware Stack

Requests автоматически логируются через middleware:

1. **CorrelationIDMiddleware** - Генерирует/извлекает correlation ID
2. **RequestLoggingMiddleware** - Логирует request/response + timing
3. **ErrorLoggingMiddleware** - Логирует ошибки и exceptions

### Request Log Example

```json
{
  "timestamp": "2026-01-10T15:30:45.123Z",
  "level": "INFO",
  "logger": "api.middleware",
  "message": "HTTP request completed",
  "correlation_id": "a1b2c3d4...",
  "method": "POST",
  "path": "/api/v1/tokens",
  "query_params": {},
  "status_code": 201,
  "duration_ms": 42.5,
  "client_host": "192.168.1.100",
  "user_agent": "Mozilla/5.0..."
}
```

## Critical Operations Logging

### Token Operations

```python
# Create
logger.info("Token created", extra={"token_id": id, "user_id": user_id})

# Read
logger.debug("Token retrieved", extra={"token_id": id})

# Update
logger.info("Token updated", extra={"token_id": id, "fields": fields})

# Delete
logger.warning("Token deleted", extra={"token_id": id, "reason": reason})
```

### Grid Operations

```python
# Spatial query
logger.info(
    "Grid spatial query",
    extra={
        "query_type": "neighbors",
        "coordinates": coords,
        "radius": radius,
        "result_count": len(results),
        "duration_ms": duration
    }
)
```

### WebSocket Operations

```python
# Connection
logger.info(
    "WebSocket connected",
    extra={
        "client_id": client_id,
        "user_id": user_id,
        "total_connections": manager.connection_count
    }
)

# Message sent
logger.debug(
    "WebSocket message sent",
    extra={
        "client_id": client_id,
        "channel": channel,
        "message_type": msg_type,
        "size_bytes": size
    }
)
```

### Authentication & Security

```python
# Login attempt
logger.info(
    "Login attempt",
    extra={
        "username": username,
        "ip_address": ip,
        "user_agent": user_agent
    }
)

# Rate limit exceeded
logger.warning(
    "Rate limit exceeded",
    extra={
        "ip_address": ip,
        "endpoint": endpoint,
        "limit": limit,
        "current": current
    }
)

# Security event
logger.error(
    "Suspicious activity detected",
    extra={
        "event_type": "sql_injection_attempt",
        "ip_address": ip,
        "payload": sanitized_payload
    }
)
```

## Log Aggregation & Analysis

### Elasticsearch Query Examples

```json
// Find all errors in last hour
GET /neurograph-logs/_search
{
  "query": {
    "bool": {
      "must": [
        {"term": {"level": "ERROR"}},
        {"range": {"timestamp": {"gte": "now-1h"}}}
      ]
    }
  }
}

// Trace request by correlation ID
GET /neurograph-logs/_search
{
  "query": {
    "term": {"correlation_id": "a1b2c3d4..."}
  },
  "sort": [{"timestamp": "asc"}]
}

// Find slow requests (>1s)
GET /neurograph-logs/_search
{
  "query": {
    "range": {"duration_ms": {"gte": 1000}}
  }
}
```

### Grafana Loki Queries

```logql
# All errors
{service="api"} |= "ERROR"

# Specific component errors
{service="api", component="tokens"} |= "ERROR"

# Slow requests
{service="api"} | json | duration_ms > 1000

# Trace by correlation ID
{service="api"} | json | correlation_id="a1b2c3d4..."
```

## Best Practices

### ✅ DO

- Use structured extra fields instead of string interpolation
- Include relevant context (user_id, token_id, etc.)
- Log at appropriate levels (INFO for important, DEBUG for detailed)
- Use correlation IDs for request tracing
- Log before and after critical operations
- Include timing/performance metrics

### ❌ DON'T

- Log sensitive data (passwords, API keys, tokens)
- Log PII without anonymization
- Use string formatting for structured data
- Log excessive detail in production (use DEBUG level)
- Log in tight loops (aggregate instead)
- Include raw stack traces in responses (log only)

### Security Considerations

```python
# ❌ BAD - Logs password
logger.info(f"User login: {username} / {password}")

# ✅ GOOD - No sensitive data
logger.info("User login attempt", extra={"username": username})

# ❌ BAD - Logs full token
logger.info(f"Token: {token}")

# ✅ GOOD - Logs only token ID
logger.info("Token validated", extra={"token_id": token_id})
```

## Performance Considerations

- JSON serialization has ~5-10μs overhead
- Correlation ID lookup is async-safe (ContextVar)
- Skip logging for health checks (noise reduction)
- Use appropriate log levels (DEBUG = verbose, INFO = default)
- Consider log sampling for high-traffic endpoints

## Troubleshooting

### Logs not appearing

```bash
# Check log level
echo $LOG_LEVEL

# Check if JSON format is enabled
echo $LOG_JSON_FORMAT

# Test logging manually
python -c "from api.logging_config import setup_logging, get_logger; \
  setup_logging('DEBUG', True); \
  logger = get_logger('test'); \
  logger.info('Test log')"
```

### Missing correlation IDs

```bash
# Verify middleware is enabled (check main.py)
grep -n "CorrelationIDMiddleware" src/api/main.py

# Test with curl
curl -H "X-Correlation-ID: test-123" http://localhost:8080/api/v1/health
```

### Parsing JSON logs

```bash
# Pretty print logs
docker-compose logs neurograph-api | jq '.'

# Filter by level
docker-compose logs neurograph-api | jq 'select(.level=="ERROR")'

# Extract correlation IDs
docker-compose logs neurograph-api | jq -r '.correlation_id' | sort -u
```

## Version History

- **v0.67.0** - Enhanced documentation
- **v0.58.0** - Added security event logging
- **v0.52.0** - Initial structured logging implementation

## See Also

- [Prometheus Metrics Guide](../metrics/PROMETHEUS.md)
- [Grafana Dashboards](../../grafana/README.md)
- [Health Endpoints](../api/HEALTH.md)
