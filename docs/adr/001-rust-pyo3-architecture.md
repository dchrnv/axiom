# ADR-001: Rust Core + PyO3 Architecture

**Status:** Accepted

**Date:** 2024-11-29

**Deciders:** Chernov Denys

## Context

Axiom requires high-performance spatial computations for 8-dimensional token operations. Python alone cannot achieve the required performance (< 1μs latency for core operations), especially for:
- Distance calculations in 8D space
- Neighbor finding algorithms
- Grid cell lookups
- Connection calculations

We needed to decide on a technology stack that balances:
- Performance (computational speed)
- Developer productivity (Python ecosystem)
- Memory efficiency
- Maintainability

## Decision

We will use **Rust for computational core** with **PyO3 bindings** for Python integration.

**Architecture:**
- `src/core/` - Rust implementation of spatial algorithms
- `src/api/` - Python FastAPI application
- PyO3 - FFI bindings between Rust and Python

**Key components in Rust:**
- Token distance calculations
- Grid spatial indexing
- Connection finding (neighbor search)
- Field influence calculations

**Key components in Python:**
- REST API (FastAPI)
- WebSocket server
- Authentication/authorization
- Storage layer (in-memory)
- Business logic orchestration

## Consequences

### Positive

✅ **Performance:** 50-100x speedup for core operations
- Distance calculations: ~0.39μs (vs ~10μs pure Python)
- Connection finding with caching: ~50x faster
- Grid operations: O(1) cell lookup

✅ **Type Safety:** Rust's strong type system prevents bugs at compile time

✅ **Memory Safety:** No garbage collection overhead, predictable performance

✅ **Ecosystem Access:** Full Python ecosystem for API/web/data science

✅ **Gradual Migration:** Can move more logic to Rust incrementally

### Negative

❌ **Build Complexity:** Requires Rust toolchain for development

❌ **Learning Curve:** Contributors need Rust knowledge for core changes

❌ **FFI Overhead:** Small overhead for Python-Rust calls (~100ns)
   - Mitigated by batching operations and caching

❌ **Debugging:** Cross-language debugging more complex

❌ **Deployment:** Need to compile Rust for target platforms

## Alternatives Considered

### 1. Pure Python with NumPy

**Pros:**
- Simplest development
- Largest contributor pool
- Easy deployment

**Cons:**
- Too slow for target performance (~10-50x slower)
- GIL limitations for parallelism
- Memory overhead

**Rejected because:** Cannot meet < 1μs performance target

### 2. C++ with pybind11

**Pros:**
- Mature ecosystem
- Excellent performance
- Good Python integration

**Cons:**
- Memory safety issues (manual management)
- Undefined behavior risks
- Harder to maintain

**Rejected because:** Rust provides similar performance with better safety guarantees

### 3. Cython

**Pros:**
- Python-like syntax
- Good NumPy integration
- Easier learning curve

**Cons:**
- Still slower than Rust
- Limited type safety
- Less predictable performance

**Rejected because:** Performance not competitive with Rust

### 4. Julia

**Pros:**
- High performance
- Scientific computing focus
- Dynamic typing

**Cons:**
- Smaller ecosystem
- Less mature Python integration
- Harder deployment

**Rejected because:** PyO3 more mature than PyJulia

## Implementation

### Phase 1: Core Algorithms (Completed)
- ✅ Distance calculations
- ✅ Connection finding
- ✅ Grid operations
- ✅ PyO3 bindings

### Phase 2: Optimization (Completed)
- ✅ Connection result caching (~50x speedup)
- ✅ Memory layout optimization
- ✅ SIMD vectorization where applicable

### Phase 3: Future Enhancements
- ⏳ GPU acceleration (CUDA/ROCm) for batch operations
- ⏳ Parallel grid queries
- ⏳ Memory-mapped storage for large datasets

## Performance Benchmarks

```
Operation               Pure Python    Rust + PyO3    Speedup
-----------------------------------------------------------------
Distance (8D)           ~10.0μs        ~0.39μs        ~26x
Find connections        ~500ms         ~10ms (cached) ~50x
Grid lookup             ~5.0μs         ~0.1μs         ~50x
```

## References

- [PyO3 Documentation](https://pyo3.rs/)
- [Rust FFI Performance](https://doc.rust-lang.org/nomicon/ffi.html)
- Axiom Performance Benchmarks (internal)

## Revision History

- 2024-11-29: Initial decision
- 2024-12-15: Added performance benchmarks
- 2026-01-09: Updated with production experience
