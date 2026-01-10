# Axiom Python Client

Official Python client library for [Axiom](https://github.com/dchrnv/axiom-os) - semantic knowledge system based on token embeddings.

[![Python Version](https://img.shields.io/pypi/pyversions/axiom-python)](https://pypi.org/project/axiom-python/)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg)](LICENSE)

## Features

- ✅ **Synchronous and Asynchronous** clients
- ✅ **Type hints** with Pydantic models
- ✅ **JWT and API Key** authentication
- ✅ **Automatic token refresh** for JWT
- ✅ **Comprehensive error handling**
- ✅ **Full API coverage** (tokens, API keys, health checks)
- ✅ **Context manager** support
- ✅ **Timeout** and connection management

## Installation

```bash
pip install axiom-python
```

For development:
```bash
git clone https://github.com/dchrnv/axiom-os.git
cd axiom-os/python-client
pip install -e ".[dev]"
```

## Quick Start

### Synchronous Client

```python
from axiom import AxiomClient

# Initialize with JWT authentication
client = AxiomClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)

# Create a token
token = client.tokens.create(text="hello world")
print(f"Token ID: {token.id}")
print(f"Embedding: {token.embedding[:5]}...")

# Query similar tokens
results = client.tokens.query(
    query_vector=token.embedding,
    top_k=10
)
for result in results:
    print(f"{result.token.text}: {result.similarity:.4f}")

# Close client
client.close()
```

### Asynchronous Client

```python
import asyncio
from axiom import AsyncAxiomClient

async def main():
    async with AsyncAxiomClient(
        base_url="http://localhost:8000",
        api_key="ng_your_api_key_here"
    ) as client:
        # Create tokens concurrently
        tasks = [
            client.tokens.create(text=f"token {i}")
            for i in range(10)
        ]
        tokens = await asyncio.gather(*tasks)
        print(f"Created {len(tokens)} tokens")

asyncio.run(main())
```

## Authentication

### JWT Authentication (Username/Password)

```python
client = AxiomClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)
```

**Default users:**
- `admin` / `admin123` - Full access
- `developer` / `developer123` - Read + Write
- `viewer` / `viewer123` - Read only

### API Key Authentication

```python
# First, create an API key using JWT
client = AxiomClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)

api_key = client.api_keys.create(
    name="My Integration",
    scopes=["tokens:read", "tokens:write"],
    expires_in_days=30
)
print(f"Save this key: {api_key.api_key}")

# Then use the API key
client = AxiomClient(
    base_url="http://localhost:8000",
    api_key=api_key.api_key
)
```

## API Reference

### Tokens

```python
# Create token
token = client.tokens.create(
    text="hello world",
    metadata={"category": "greeting"}
)

# Get token by ID
token = client.tokens.get(token_id=123)

# List tokens
tokens = client.tokens.list(limit=100, offset=0)

# Update token
updated = client.tokens.update(
    token_id=123,
    text="new text",
    metadata={"updated": True}
)

# Delete token
client.tokens.delete(token_id=123)

# Query similar tokens
results = client.tokens.query(
    query_vector=[0.1, 0.2, ...],
    top_k=10,
    threshold=0.8
)
```

### API Keys

```python
# Create API key (returns full key - save it!)
api_key = client.api_keys.create(
    name="Integration Key",
    scopes=["tokens:read", "tokens:write"],
    expires_in_days=30
)

# List API keys
keys = client.api_keys.list()

# Get API key details
key = client.api_keys.get(key_id="key_123")

# Revoke API key
client.api_keys.revoke(key_id="key_123")

# Delete API key
client.api_keys.delete(key_id="key_123")
```

### Health Checks

```python
# Check API health
health = client.health.check()
print(health.status, health.version)

# Get system status
status = client.health.status()
print(f"Tokens: {status.tokens_count}")
print(f"Uptime: {status.uptime_seconds}s")
```

## Error Handling

```python
from axiom import (
    AxiomError,
    AuthenticationError,
    AuthorizationError,
    NotFoundError,
    ValidationError,
    RateLimitError,
    ServerError,
)

try:
    token = client.tokens.get(token_id=999999)
except NotFoundError as e:
    print(f"Token not found: {e.message}")
except AuthenticationError as e:
    print(f"Auth failed: {e.message}")
except RateLimitError as e:
    print(f"Rate limit exceeded. Retry after {e.retry_after}s")
except AxiomError as e:
    print(f"Error [{e.error_code}]: {e.message}")
```

## Context Manager

```python
# Sync client
with AxiomClient(...) as client:
    token = client.tokens.create(text="hello")
    # Client automatically closed on exit

# Async client
async with AsyncAxiomClient(...) as client:
    token = await client.tokens.create(text="hello")
    # Client automatically closed on exit
```

## Configuration

```python
client = AxiomClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123",
    timeout=30.0,           # Request timeout in seconds
    verify_ssl=True,        # Verify SSL certificates
)
```

## Examples

See the [examples/](examples/) directory for more examples:

- [basic_usage.py](examples/basic_usage.py) - Comprehensive synchronous examples
- [async_usage.py](examples/async_usage.py) - Asynchronous patterns

## Requirements

- Python 3.10+
- httpx >= 0.25.0
- pydantic >= 2.0.0

## Development

```bash
# Clone repository
git clone https://github.com/dchrnv/axiom-os.git
cd axiom-os/python-client

# Install with dev dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Run linters
black axiom/
ruff check axiom/
mypy axiom/
```

## API Compatibility

This client is compatible with Axiom API **v0.58.0+**

Features by version:
- **v0.58.0**: JWT auth, API keys, RBAC, rate limiting
- **v0.59.0**: Python client library (this package)

## License

AGPLv3 - See [LICENSE](../LICENSE) for details

## Links

- **Documentation**: https://axiom.dev/docs
- **GitHub**: https://github.com/dchrnv/axiom-os
- **Issues**: https://github.com/dchrnv/axiom-os/issues
- **PyPI**: https://pypi.org/project/axiom-python/

## Support

- GitHub Issues: https://github.com/dchrnv/axiom-os/issues
- GitHub Discussions: https://github.com/dchrnv/axiom-os/discussions

---

**Built with ❤️ by the Axiom team**
