# Contributing to NeuroGraph

Thank you for your interest in contributing to NeuroGraph! This document provides guidelines and standards for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Style Guidelines](#code-style-guidelines)
- [Testing Requirements](#testing-requirements)
- [Documentation Standards](#documentation-standards)
- [Pull Request Process](#pull-request-process)
- [Architecture Decision Records](#architecture-decision-records)

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Maintain professional communication

## Getting Started

### Prerequisites

- Python 3.11+
- Rust 1.70+
- Git

### Setup Development Environment

```bash
# Clone repository
git clone https://github.com/chrnv/neurograph-os-mvp.git
cd neurograph-os-mvp

# Install Python dependencies
pip install -r requirements.txt
pip install -r requirements-dev.txt

# Build Rust core
cd src/core
cargo build --release
cd ../..

# Run tests
pytest tests/ -v

# Start development server
./run.sh
```

## Development Workflow

### 1. Create a Branch

```bash
# Feature branch
git checkout -b feature/your-feature-name

# Bug fix branch
git checkout -b fix/issue-description

# Documentation branch
git checkout -b docs/what-you-are-documenting
```

### 2. Make Changes

- Write clean, readable code
- Follow style guidelines (see below)
- Add tests for new functionality
- Update documentation as needed

### 3. Run Quality Checks

```bash
# Python linting
ruff check src/api/
mypy src/api/

# Rust linting
cd src/core
cargo clippy -- -D warnings
cargo fmt --check

# Run tests
pytest tests/ -v --cov=src/api

# Check test coverage
pytest --cov=src/api --cov-report=html
```

### 4. Commit Changes

Follow conventional commit format:

```bash
# Format: <type>(<scope>): <description>

# Examples:
git commit -m "feat(api): add token batch creation endpoint"
git commit -m "fix(grid): resolve neighbor search boundary issue"
git commit -m "docs(tutorials): add performance optimization guide"
git commit -m "test(tokens): add comprehensive CRUD tests"
git commit -m "refactor(storage): optimize memory usage"
```

**Commit Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test additions or modifications
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `style`: Code style changes (formatting)
- `chore`: Build process, dependencies

### 5. Push and Create PR

```bash
git push origin your-branch-name
```

Then create a Pull Request on GitHub.

## Code Style Guidelines

### Python (PEP 8 + Project Standards)

**Formatting:**
- Use `ruff` for linting and formatting
- Line length: 100 characters
- Indentation: 4 spaces
- Use type hints for all functions

**Example:**

```python
from typing import Optional, List

def create_token(
    position: List[float],
    radius: float = 1.0,
    weight: float = 1.0
) -> Optional[int]:
    """Create a new token.

    Args:
        position: 8D coordinate vector
        radius: Token influence radius
        weight: Token importance factor

    Returns:
        Token ID if successful, None otherwise

    Example:
        >>> token_id = create_token([0.0] * 8, radius=2.0)
        >>> print(token_id)
        1
    """
    # Implementation
    pass
```

**Docstrings:**
- Use Google style
- Include Args, Returns, Raises, Example sections
- Document all public functions and classes

**Naming Conventions:**
- Functions/variables: `snake_case`
- Classes: `PascalCase`
- Constants: `UPPER_SNAKE_CASE`
- Private members: `_leading_underscore`

### Rust (Rust Style Guide)

**Formatting:**
- Use `cargo fmt` (rustfmt)
- Use `cargo clippy` for linting

**Example:**

```rust
/// Calculate distance between two points in N-dimensional space.
///
/// # Arguments
/// * `p1` - First point coordinates
/// * `p2` - Second point coordinates
///
/// # Returns
/// Euclidean distance between points
///
/// # Example
/// ```
/// let dist = calculate_distance(&[0.0, 0.0], &[3.0, 4.0]);
/// assert_eq!(dist, 5.0);
/// ```
pub fn calculate_distance(p1: &[f64], p2: &[f64]) -> f64 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}
```

**Naming Conventions:**
- Functions/variables: `snake_case`
- Types/Traits: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Lifetimes: `'short_lowercase`

### JavaScript/TypeScript

- Use Prettier for formatting
- ESLint for linting
- Follow Airbnb style guide

## Testing Requirements

### Test Coverage Targets

- **Critical paths**: 80%+ coverage required
- **API endpoints**: 70%+ coverage required
- **Core logic**: 90%+ coverage required

### Writing Tests

**Python (pytest):**

```python
import pytest
from src.api.storage.memory import InMemoryTokenStorage

class TestTokenStorage:
    """Test suite for token storage operations."""

    @pytest.fixture
    def storage(self):
        """Create fresh storage instance."""
        return InMemoryTokenStorage()

    def test_create_token(self, storage):
        """Test token creation."""
        token = storage.create(
            position=[1.0] * 8,
            radius=1.0,
            weight=1.0
        )
        assert token is not None
        assert token.radius == 1.0

    def test_get_nonexistent_token(self, storage):
        """Test retrieving non-existent token."""
        token = storage.get(999)
        assert token is None
```

**Rust (built-in test framework):**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_calculation() {
        let p1 = vec![0.0, 0.0];
        let p2 = vec![3.0, 4.0];
        let dist = calculate_distance(&p1, &p2);
        assert!((dist - 5.0).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "mismatched dimensions")]
    fn test_distance_dimension_mismatch() {
        let p1 = vec![0.0, 0.0];
        let p2 = vec![1.0, 2.0, 3.0];
        calculate_distance(&p1, &p2);
    }
}
```

### Running Tests

```bash
# Python tests
pytest tests/ -v

# With coverage
pytest tests/ --cov=src/api --cov-report=html

# Specific test file
pytest tests/test_tokens.py -v

# Rust tests
cd src/core
cargo test
cargo test --release  # Release mode

# With output
cargo test -- --nocapture
```

## Documentation Standards

### Code Documentation

- **Python**: Google-style docstrings
- **Rust**: Rustdoc comments (`///`)
- **All public APIs**: Must be documented

### User Documentation

- **Location**: `docs/source/`
- **Format**: reStructuredText (.rst) or Markdown (.md)
- **Build**: `make html` in `docs/` directory

### Tutorial Documentation

- **Location**: `docs/tutorials/`
- **Format**: Jupyter notebooks (.ipynb)
- **Requirements**: Runnable examples, clear explanations

### Updating Documentation

```bash
# After changes, rebuild docs
cd docs
make clean
make html

# Check for warnings
# Fix any broken links or missing references
```

## Pull Request Process

### Before Submitting

1. ✅ All tests pass
2. ✅ Code coverage meets requirements
3. ✅ Linting passes (ruff, clippy)
4. ✅ Documentation updated
5. ✅ CHANGELOG.md updated (if applicable)

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe testing performed

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] Linting passes
- [ ] No breaking changes (or documented)

## Related Issues
Fixes #123
```

### Review Process

1. **Automated checks** run on PR
2. **Maintainer review** (usually within 48 hours)
3. **Address feedback** if any
4. **Approval and merge**

### Merge Requirements

- ✅ At least 1 approval from maintainer
- ✅ All CI checks passing
- ✅ No merge conflicts
- ✅ Branch up-to-date with main

## Architecture Decision Records

When making significant architectural decisions, document them as ADRs in `docs/adr/`.

**Template:**

```markdown
# ADR-NNN: Title

**Status:** Proposed | Accepted | Deprecated | Superseded

**Date:** YYYY-MM-DD

## Context
What is the issue we're facing?

## Decision
What decision did we make?

## Consequences
What are the positive and negative consequences?

## Alternatives Considered
What other options were considered?
```

**Existing ADRs:**
- [ADR-001: Rust Core + PyO3 Architecture](docs/adr/001-rust-pyo3-architecture.md)
- [ADR-002: 8-Dimensional Coordinate System](docs/adr/002-8d-coordinate-system.md)
- [ADR-003: WebSocket Event Streaming](docs/adr/003-websocket-streaming.md)
- [ADR-004: Token-Based Cognitive Architecture](docs/adr/004-token-architecture.md)
- [ADR-005: CDNA Profile System](docs/adr/005-cdna-profiles.md)

## Questions or Issues?

- **Bug reports**: [GitHub Issues](https://github.com/chrnv/neurograph-os-mvp/issues)
- **Feature requests**: [GitHub Discussions](https://github.com/chrnv/neurograph-os-mvp/discussions)
- **Security issues**: Email security@neurograph.dev

## License

By contributing, you agree that your contributions will be licensed under the GNU AGPL v3 License.

---

**Thank you for contributing to NeuroGraph!** 🚀
