# ADR-002: 8-Dimensional Coordinate System

**Status:** Accepted

**Date:** 2024-11-15

**Deciders:** Chernov Denys

## Context

NeuroGraph tokens exist in a multi-dimensional space representing cognitive relationships. We needed to determine:
- How many dimensions to support
- Whether dimension count should be fixed or variable
- How to represent positions in code
- Performance implications of dimension count

## Decision

We will use a **fixed 8-dimensional coordinate system** for all tokens.

**Representation:**
- Position: `[d0, d1, d2, d3, d4, d5, d6, d7]` (array of 8 floats)
- Dimensions are named d0-d7 (not semantically labeled)
- All spatial operations work in 8D space

**CDNA (Cognitive DNA) provides dimensional scaling:**
- Each dimension has a scale multiplier
- Profiles adjust dimensional importance
- Enables behavioral adaptation without changing core algorithms

## Consequences

### Positive

✅ **Consistent Representation:** All code assumes 8D, simplifying algorithms

✅ **Performance:** Fixed size enables:
- Stack allocation (no heap overhead)
- SIMD optimization (8 floats fit in AVX-256)
- Cache-friendly memory layout

✅ **Cognitive Modeling:** 8 dimensions sufficient for:
- Spatial relationships (3D)
- Temporal aspects (1-2D)
- Abstract properties (3-4D)

✅ **CDNA Flexibility:** Dimension scales provide runtime adaptability

### Negative

❌ **Fixed Dimensionality:** Cannot add/remove dimensions at runtime

❌ **Memory Usage:** Always allocates 8 floats even if fewer needed
- Mitigated: 64 bytes per token is acceptable

❌ **Mathematical Overhead:** Distance calculations always compute 8 components
- Mitigated: Negligible with SIMD

## Alternatives Considered

### 1. Variable Dimensionality

**Pros:**
- Flexible for different use cases
- Can optimize memory for lower dimensions

**Cons:**
- Generic code complex (template/trait bounds)
- Dynamic allocation overhead
- Cache misses from pointer chasing
- No SIMD optimization

**Rejected because:** Performance penalty unacceptable

### 2. 3D Space (Traditional)

**Pros:**
- Simpler visualization
- Familiar to developers
- Lower memory usage

**Cons:**
- Insufficient for cognitive modeling
- Cannot represent abstract relationships
- Limits future extensibility

**Rejected because:** Cognitive architecture requires more dimensions

### 3. 16 or 32 Dimensions

**Pros:**
- More expressive power
- Future-proof

**Cons:**
- Memory overhead (128-256 bytes per token)
- Slower distance calculations
- No current use case for >8D

**Rejected because:** 8D balances expressiveness and performance

### 4. Sparse Representation

**Pros:**
- Memory efficient for few active dimensions
- Flexible dimension usage

**Cons:**
- Complex indexing
- Cache inefficiency
- Slower computations

**Rejected because:** Not a good fit for dense cognitive space

## Implementation

### Data Structures

**Python:**
```python
position: List[float]  # Always length 8
# Validated at API layer
```

**Rust:**
```rust
pub type Position = [f64; 8];  // Fixed-size array
```

### Validation

API layer enforces:
- Exactly 8 coordinates required
- All values must be finite (no NaN/Inf)
- Type: floating-point numbers

### CDNA Dimension Scales

Profiles adjust effective dimensionality:
```python
scales = [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0]
# Dimension 7 has 5x weight compared to dimension 0
```

## Performance Impact

**Memory per token:** 64 bytes (8 × 8 bytes)

**Distance calculation:** ~0.39μs
```rust
// Optimized with SIMD when available
pub fn distance(p1: &[f64; 8], p2: &[f64; 8]) -> f64 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}
```

**Grid cell computation:** O(1) with 8 dimensions

## Future Considerations

### If dimensionality needs change:

1. **Semantic Dimensions** (v2.0):
   - Label dimensions with meaning (spatial, temporal, abstract)
   - Provide named access (position.spatial, position.temporal)
   - Maintain 8D core for compatibility

2. **Hierarchical Dimensions** (v3.0):
   - Group dimensions into categories
   - Enable selective queries on subsets
   - Still use 8D representation internally

3. **Dimension Expansion** (breaking change):
   - Would require new ADR
   - Migration path for existing data
   - Performance re-evaluation

## References

- Cognitive Science: Multi-dimensional semantic spaces
- Vector databases: Typical dimensionality (512-2048)
- NeuroGraph: Balance between expressiveness and performance

## Revision History

- 2024-11-15: Initial decision
- 2024-12-10: Added CDNA scaling context
- 2026-01-09: Performance measurements added
