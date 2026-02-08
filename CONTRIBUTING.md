# Contributing to NeuroGraph

Thank you for your interest in contributing to NeuroGraph!

## Development Setup

### 1. Install Dependencies

```bash
# Python dependencies
pip install -r requirements.txt
pip install -r src/api/requirements.txt
pip install -r tests/requirements.txt

# Development tools
pip install pre-commit black isort flake8 mypy bandit safety

# Rust (if modifying core)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Set Up Pre-commit Hooks

```bash
# Install pre-commit hooks
pre-commit install

# Run on all files (first time)
pre-commit run --all-files
```

## Pre-commit Hooks

We use pre-commit hooks to ensure code quality and security:

### Security Checks
- **bandit** - Python security linter
- **safety** - Dependency vulnerability checker
- **detect-secrets** - Prevents committing secrets
- **detect-private-key** - Prevents committing private keys

### Code Quality
- **black** - Python code formatter
- **isort** - Import sorter
- **flake8** - Python linter
- **mypy** - Type checker
- **cargo fmt** - Rust formatter
- **cargo clippy** - Rust linter

### Standard Checks
- Trailing whitespace
- End of file fixer
- YAML/JSON validation
- Large files check
- Merge conflict detection

## Running Tests

```bash
# Python API tests
pytest tests/

# Rust tests
cd src/core_rust && cargo test

# Performance benchmarks
python tests/performance/stress_benchmark.py 1m
python tests/performance/rest_api_benchmark.py 1m
```

## Code Style

### Python
- Line length: 100 characters
- Formatter: black
- Import style: isort with black profile
- Type hints: preferred but not required

### Rust
- Standard rustfmt configuration
- Clippy warnings: treated as errors in CI

## Security Guidelines

1. **Never commit secrets** - use environment variables
2. **Use bandit** for security scanning before PR
3. **Run safety check** for dependency vulnerabilities
4. **Follow OWASP Top 10** best practices
5. **Add security tests** for authentication/authorization features

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run pre-commit checks (`pre-commit run --all-files`)
5. Run tests (`pytest && cargo test`)
6. Commit changes (`git commit -m 'feat: Add amazing feature'`)
7. Push to branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Commit Message Format

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding tests
- `chore`: Build/tooling changes

Example:
```
feat(api): Add token batch creation endpoint

Implements batch token creation for improved performance.
Includes rate limiting and input validation.

Closes #123
```

## Reporting Security Vulnerabilities

Please see [SECURITY.md](../SECURITY.md) for our security policy.

## Questions?

- GitHub Issues: [Create an issue](https://github.com/dchrnv/neurograph/issues/new)
- Discussions: [Join the discussion](https://github.com/dchrnv/neurograph/discussions)

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0 License.
