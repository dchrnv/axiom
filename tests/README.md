# Axiom Test Suite

## Structure

```
tests/
├── unit/               # Unit tests (no external dependencies)
│   ├── api/           # API layer unit tests
│   └── conftest.py    # Unit test fixtures
│
├── integration/       # Integration tests (no Rust Core)
│   └── conftest.py    # Integration fixtures
│
├── e2e/               # End-to-end tests (require Rust Core)
│   └── conftest.py    # E2E fixtures
│
└── performance/       # Performance benchmarks
```

## Running Tests

```bash
# Unit tests only (fast)
pytest tests/unit/ -v

# All tests except E2E
pytest tests/ --ignore=tests/e2e/ -v

# E2E tests (requires Rust Core)
pytest tests/e2e/ -v
```
