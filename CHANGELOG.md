# Changelog

All notable changes to Axiom OS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v 1.00.00] - 2026-01-10

### Added

- **Project Rebranding**: Officially renamed from "NeuroGraph" to "Axiom".
- **GitHub Connection**: Connected to the new repository `https://github.com/dchrnv/axiom`.
- **Version Reset**: Reset project version to `v 1.00.00` to mark the start of the Axiom project from scratch.
- **Clean History**: Initialized a fresh Git repository for a clean start.

## [Unreleased]

### Added

- **GitHub Actions CI workflow** for main project (`.github/workflows/main-ci.yml`)
  - Python tests (pytest) for Python 3.10, 3.11, 3.12
  - Rust tests (cargo test, clippy, format check)
  - Python linting (ruff, mypy)
  - Coverage reporting to Codecov
  - Build check with maturin
- **PyPI packaging configuration**
  - Enhanced `pyproject.toml` with optional dependencies (`jupyter`, `api`, `all`, `dev`)
  - `MANIFEST.in` for proper file inclusion
  - CLI entry point (`axiom` command)
  - Jupyter extension entry points
- **Documentation improvements**
  - PyPI Publishing Guide (`docs/guides/PYPI_PUBLISHING_GUIDE.md`)
  - Jupyter CLI tool (`src/axiom_jupyter/cli.py`)

### Changed

- Updated MASTER_PLAN from v3.1 to v3.2 with accurate status tracking
- Updated DEVELOPMENT_PLAN from v1.0 to v1.1 with realistic progress assessment (80% complete)
- Enhanced `pyproject.toml` metadata for PyPI publication
- Bumped `requires-python` to >=3.10 (dropped 3.8, 3.9 support)

### Fixed

- Corrected documentation claims about Jupyter integration (was incorrectly listed as "not implemented")
- Fixed Module Registry status (was incorrectly listed as "uncommitted")

## [0.63.1] - 2024-12-31

### Added

- Comprehensive Development Plan and Roadmap documentation
- Project structure reorganization for better clarity

### Changed

- Bumped version to v0.63.1
- Updated documentation to reflect actual implementation state

## [0.63.0] - 2024-12-30

### Added

- **Module Registry System** - Complete implementation
  - API endpoints for module management (`GET /modules`, `GET /modules/{id}`, etc.)
  - Enable/disable functionality for all modules
  - Metrics tracking per module
  - Configuration management API
  - Integration with Rust Core components
- Rust Core module identification (`module_id.rs`)
- Python bindings for module management

### Changed

- Updated MASTER_PLAN from v3.0 to v3.1
- Refocused roadmap on Module Registry completion

## [0.62.0] - 2024-12-29

### Added

- **Complete Web Dashboard (React SPA)**
  - 7 functional pages (Dashboard, Modules, Config, Bootstrap, Chat, Terminal, Admin)
  - 35+ TypeScript/TSX files with 3,512+ lines of code
  - 15+ reusable UI components
  - 4 Zustand stores with localStorage persistence
  - Real-time WebSocket communication
  - Full internationalization (EN/RU) with 160+ translation keys
  - Dark/Light theme support
  - Error boundaries and 404 handling
  - Connection status monitoring
  - Responsive design for all screen sizes
  - Automation scripts (`./start-all.sh`, `./stop-all.sh`)

### Performance

- Load time: < 2s
- Time to interactive: < 3s
- Lighthouse score: > 90
- Mobile responsive: 100%

## [0.61.1] - 2024-12-28

### Added

- **IPython Widgets** for live monitoring in Jupyter notebooks
- **Unit tests** for Jupyter integration (13KB test file)
- Enhanced Jupyter guide with v0.61.1 features

### Fixed

- Minor bugs in Jupyter magic commands
- Display formatting issues

## [0.61.0] - 2024-12-27

### Added

- **Complete Jupyter Integration**
  - IPython extension with magic commands (`%axiom`)
  - 7 core modules:
    - `magic.py` - Magic commands implementation
    - `display.py` - Rich HTML display formatters
    - `helpers.py` - DataFrame conversion and export utilities
    - `query_builder.py` - Natural language query builder
    - `widgets.py` - Interactive Jupyter widgets
    - `plotly_viz.py` - Plotly-based visualizations
    - `__init__.py` - Extension loader
  - Magic commands:
    - `%axiom init` - Initialize connection
    - `%axiom status` - Show system status
    - `%axiom query` - Execute queries
    - `%axiom subscribe` - Subscribe to channels
    - `%axiom emit` - Emit events
  - DataFrame helpers:
    - `to_dataframe()` - Convert results to pandas DataFrame
    - `export_csv()` - Export to CSV
    - `plot_distribution()` - Visualize data distribution
  - Rich HTML output formatting
  - Auto-completion support

### Documentation

- Full Jupyter integration tutorial
- README updated with installation and usage examples

## [0.60.1] - 2024-12-26

### Added

- **WebSocket CLI Tool** (`src/api/websocket/cli.py`)
  - Interactive command-line interface for WebSocket testing
  - Support for subscribe, broadcast, ping commands
  - Real-time event monitoring
  - Full documentation in README

## [0.60.0] - 2024-12-25

### Added

- **Complete WebSocket Support** (12 modules)
  - Real-time bidirectional communication
  - Event channels system (metrics, signals, actions, logs, status, connections)
  - RBAC permissions system (5 roles: Admin, Developer, Viewer, Bot, Anonymous)
  - Token bucket rate limiting with configurable limits per message type
  - Binary message support for efficient data transfer
  - Compression support (gzip, deflate, brotli)
  - Reconnection tokens for seamless session recovery (5-minute expiry)
  - Connection lifecycle management
  - Prometheus metrics integration (15 metrics)
  - TypeScript and Python client libraries
  - WebSocket endpoint at `/ws`

### Modules Structure

- `manager.py` - Core WebSocket manager
- `connection.py` - Connection lifecycle
- `channels.py` - Channel system
- `permissions.py` - RBAC implementation
- `rate_limit.py` - Token bucket rate limiter
- `binary.py` - Binary message handling
- `compression.py` - Compression support
- `reconnection.py` - Session recovery
- `integrations.py` - Core system integration
- `metrics.py` - Prometheus metrics
- `cli.py` - CLI tool
- `__init__.py` - Module exports

### Performance

- Event delivery latency: ~5ms
- Supports 1000+ concurrent connections

## [0.57.0] - 2024-12-20

### Added

- **Complete Rust Core Intelligence**
  - Gateway v2.0 with sensor system
  - SignalSystem v1.1 with pattern matching
  - ActionController with output execution
  - Guardian system for validation
  - Full pipeline: Input → Gateway → Core → ActionController → Output

### Performance

- 304,553 events/second processing
- 0.39μs average latency
- Production-ready infrastructure

### Infrastructure

- REST API endpoints
- Prometheus metrics integration
- OpenTelemetry tracing
- Docker containerization
- PyO3 Python bindings

## Earlier Versions

See git history for detailed changes in versions prior to 0.57.0.

---

## Version Naming Convention

- **1.xx.00**: Major changes, next versions, or new phases of implementation.
- **1.00.xx**: Minor additions, patches, or refinements within a specific version or phase.
- **v 1.00.00**: Starting point for the Axiom project.

## Links

- [Repository](https://github.com/chrnv/axiom-os-mvp)
- [Documentation](./docs/)
- [Development Plan](./docs/DEVELOPMENT_PLAN.md)
- [Master Plan](./docs/MASTER_PLAN_v3.0.md)
- [PyPI Publishing Guide](./docs/guides/PYPI_PUBLISHING_GUIDE.md)
