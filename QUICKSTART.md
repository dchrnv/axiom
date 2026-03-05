# 🚀 Axiom Quick Start

> Get up and running with Axiom quickly.

---

## 📋 Prerequisites

### System Requirements
- **OS:** Linux (Arch Linux recommended, other distributions may work)
- **Python:** 3.10+
- **Rust:** 1.70+ (for development)
- **Git:** For cloning the repository

### Dependencies
- **Python packages:** Listed in `pyproject.toml`
- **Rust toolchain:** For core components
- **Neurograph:** Storage engine (automatically handled)

---

## 🛠️ Installation

### Method 1: Development Setup (Recommended)

```bash
# Clone the repository
git clone https://github.com/dchrnv/axiom.git
cd axiom

# Install Python dependencies in development mode
pip install -e ".[dev]"

# Build Rust components
cargo build --release

# Run tests to verify installation
cargo test
```

### Method 2: Production Setup

```bash
# Clone the repository
git clone https://github.com/dchrnv/axiom.git
cd axiom

# Install Python dependencies
pip install -e "."

# Build optimized Rust components
cargo build --release

# Verify installation
python -c "import axiom_core; print('Axiom installed successfully')"
```

### Method 3: Docker (Coming Soon)

```bash
# Build Docker image
docker build -t axiom .

# Run container
docker run -it axiom
```

---

## 🧪 Verify Installation

### Run Tests
```bash
# All tests
cargo test

# Specific module tests
cargo test token
cargo test domain
cargo test connection
```

### Check Sizes
```bash
# Verify structure sizes match specifications
cargo test debug_print_sizes -- --nocapture
```

### Basic Usage
```python
import axiom_core

# Create a token
token = axiom_core.Token::new(1, 1, 0)

# Create a domain
domain = axiom_core.DomainConfig::new(
    1, 
    axiom_core.DomainType::Logic, 
    axiom_core.StructuralRole::Ashti1
)

print(f"Token size: {token.size()} bytes")
print(f"Domain valid: {domain.validate()}")
```

---

## 🔧 Development Setup

### IDE Configuration
- **VS Code:** Install Rust and Python extensions
- **PyCharm:** Configure Python interpreter and Rust toolchain

### Environment Variables
```bash
# Optional: Set Rust target for optimization
export RUSTFLAGS="-C target-cpu=native"

# Optional: Enable debug symbols
export RUSTFLAGS="-g"
```

### Common Issues

#### **Issue: Rust not found**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### **Issue: Python version mismatch**
```bash
# Check Python version
python --version

# Use correct version if needed
python3.10 -m pip install -e ".[dev]"
```

#### **Issue: Build failures**
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

---

## 📚 Next Steps

1. **Read DEVELOPMENT_GUIDE.md** - for development workflow
2. **Explore docs/spec/** - for module specifications
3. **Check STATUS.md** - for current project status
4. **Join development** - see CONTRIBUTING.md

---

## 🤝 Need Help?

- **Issues:** [GitHub Issues](https://github.com/dchrnv/axiom/issues)
- **Discussions:** [GitHub Discussions](https://github.com/dchrnv/axiom/discussions)
- **Email:** dreeftwood@gmail.com

---

**Ready to dive deeper?** See [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md) for the complete development workflow.
