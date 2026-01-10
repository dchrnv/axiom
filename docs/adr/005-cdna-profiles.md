# ADR-005: CDNA (Cognitive DNA) Profile System

**Status:** Accepted

**Date:** 2024-12-05

**Deciders:** Chernov Denys

## Context

Axiom's 8D space needs **adaptive dimensional behavior** without changing core algorithms. Different use cases require different dimensional weightings:
- Spatial-focused: Emphasize d0-d2
- Temporal-focused: Emphasize time dimensions
- Abstract reasoning: Emphasize higher dimensions

We need a mechanism to:
- Adjust dimensional importance at runtime
- Switch between pre-defined behaviors
- Test configurations safely
- Track configuration history

## Decision

We will implement **CDNA (Cognitive DNA) profiles** - dynamic dimensional scaling with:
- **Dimension scales:** Per-dimension multipliers [s0, s1, ..., s7]
- **Named profiles:** Pre-defined configurations (explorer, creative, conservative)
- **Quarantine mode:** Safe testing environment for experimental configs
- **History tracking:** Audit log of configuration changes

**Core Concept:**
```python
adjusted_distance = sqrt(sum((pi - qi)^2 * si^2) for i in 0..7)
# where si is the scale for dimension i
```

## Consequences

### Positive

✅ **Runtime Adaptability:** Change behavior without code changes

✅ **Safety:** Quarantine mode prevents production issues

✅ **Flexibility:** Custom profiles for different scenarios

✅ **Auditability:** Full history of configuration changes

✅ **Experimentation:** Easy to test new dimensional weightings

### Negative

❌ **Complexity:** Additional configuration layer

❌ **Validation Needed:** Bad scales can cause pathological behavior

❌ **Documentation:** Users need to understand implications

## Profile Design

### Built-in Profiles

**Explorer** (default):
```python
{
  "scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
  "plasticity": 0.8,
  "evolution_rate": 0.5,
  "description": "Balanced exploration"
}
```

**Creative** (experimental):
```python
{
  "scales": [2.0, 3.0, 2.5, 4.0, 5.0, 4.0, 3.0, 8.0],
  "plasticity": 0.9,
  "evolution_rate": 0.7,
  "description": "High plasticity, experimental"
}
```

**Conservative** (stable):
```python
{
  "scales": [1.0, 1.0, 1.0, 1.5, 2.0, 1.5, 1.0, 2.5],
  "plasticity": 0.3,
  "evolution_rate": 0.2,
  "restricted": true,
  "max_change": 0.5,
  "description": "Low plasticity, predictable"
}
```

### Custom Profiles

Users can create custom profiles with validation:
- All scales > 0
- Scales in reasonable range (0.1 - 100.0)
- Optional restrictions (max_change)

## Quarantine Mode

**Purpose:** Test configuration changes safely

**Mechanism:**
1. Start quarantine (duration: 30-600 seconds)
2. Apply experimental configuration
3. Monitor metrics (memory, connections, errors)
4. Accept or reject changes

**Metrics Tracked:**
- `memory_growth` - Unexpected memory increase
- `connection_breaks` - Broken relationships
- `token_churn` - Excessive creation/deletion

**Auto-rollback conditions:**
- Memory growth > 50%
- Connection breaks > 10%
- Errors > 5%

## API Design

```python
# Get current config
GET /api/v1/cdna

# List profiles
GET /api/v1/cdna/profiles

# Switch profile
POST /api/v1/cdna/profiles/{profile_id}

# Update custom config
PUT /api/v1/cdna
{
  "dimension_scales": [1.0, 2.0, ...],
  "should_validate": true
}

# Quarantine mode
POST /api/v1/cdna/quarantine/start {"duration": 300}
POST /api/v1/cdna/quarantine/stop {"apply_changes": true}

# History
GET /api/v1/cdna/history
```

## Validation Rules

**Required:**
- Exactly 8 scale values
- All values > 0
- All values finite (no NaN/Inf)

**Warnings:**
- Scale < 0.1 (dimension nearly disabled)
- Scale > 50.0 (dimension over-emphasized)
- Large variance (max/min ratio > 100)

**Errors:**
- Scale = 0 (invalid)
- Scale = NaN or Inf (invalid)
- Wrong array length

## Performance Impact

**Memory:** +64 bytes per configuration (8 floats)

**Computation:** Minimal overhead
- Scale multiplication: ~1-2 cycles per dimension
- Total: ~10ns additional per distance calculation

**Caching:** Connection cache still effective

## Use Cases

### 1. Spatial-First Application
```python
# Emphasize first 3 dimensions (spatial)
scales = [10.0, 10.0, 10.0, 1.0, 1.0, 1.0, 1.0, 1.0]
```

### 2. Temporal Focus
```python
# Time is dimension 3
scales = [1.0, 1.0, 1.0, 20.0, 1.0, 1.0, 1.0, 1.0]
```

### 3. Abstract Reasoning
```python
# Emphasize higher dimensions
scales = [1.0, 1.0, 1.0, 1.0, 5.0, 8.0, 10.0, 15.0]
```

## Alternatives Considered

### 1. Per-Token Scales
**Rejected:** Too complex, cache inefficiency

### 2. Dimension Masking (enable/disable)
**Rejected:** Binary choice too limiting

### 3. Linear Transformations
**Rejected:** Scales simpler and more intuitive

## Future Enhancements

### v1.1: Profile Interpolation
```python
# Blend between two profiles
profile = interpolate(explorer, creative, alpha=0.3)
```

### v1.2: Adaptive Profiles
- Auto-tune scales based on usage patterns
- Machine learning optimization

### v2.0: Profile Versioning
- Version profiles with semantic versioning
- Migration paths for profile updates

## References

- Cognitive Science: Dimensional attention mechanisms
- Vector Spaces: Metric learning
- Axiom: Production usage patterns

## Revision History

- 2024-12-05: Initial decision
- 2024-12-20: Added quarantine mode
- 2026-01-09: Production validation
