# ADR-004: Token-Based Cognitive Architecture

**Status:** Accepted

**Date:** 2024-11-20

**Deciders:** Chernov Denys

## Context

NeuroGraph needs a fundamental unit for representing cognitive entities and their relationships. This unit must support:
- Spatial relationships in 8D space
- Dynamic properties (radius, weight)
- Connections to other entities
- Efficient storage and retrieval
- Flexible querying

## Decision

We will use **Tokens** as the fundamental building block of the cognitive architecture.

**Token Attributes:**
- **Position:** 8D coordinate `[d0, d1, d2, d3, d4, d5, d6, d7]`
- **Radius:** Influence sphere (spatial extent)
- **Weight:** Importance factor (affects field calculations)
- **ID:** Unique identifier
- **Connections:** Links to other tokens (implicit via spatial queries)

**Design Principles:**
- Tokens are immutable identifiers (IDs never reused)
- Position, radius, weight are mutable
- Connections are computed dynamically (not stored)
- No semantic labels (meaning emerges from relationships)

## Consequences

### Positive

✅ **Simplicity:** Single abstraction for all cognitive entities

✅ **Flexibility:** Generic enough for various use cases

✅ **Performance:** Lightweight (64 bytes core + metadata)

✅ **Spatial Queries:** Natural fit with grid indexing

✅ **Dynamic Behavior:** Properties can change without ID change

### Negative

❌ **No Built-in Semantics:** Applications must provide meaning

❌ **Dynamic Connections:** Must recompute neighbor relationships

## Alternatives Considered

### 1. Node-Edge Graph
**Rejected:** Explicit edge storage adds overhead and complexity

### 2. Tagged Entities (with semantic labels)
**Rejected:** Reduces flexibility, couples architecture to specific use cases

### 3. Hierarchical Structures (trees)
**Rejected:** Too rigid, doesn't support arbitrary relationships

## Implementation

```python
class Token:
    token_id: int          # Unique identifier
    position: List[float]  # 8D coordinates
    radius: float          # Influence radius
    weight: float          # Importance factor
    created_at: datetime
    updated_at: datetime
```

**Operations:**
- Create: Generate ID, set properties
- Read: Retrieve by ID or spatial query
- Update: Modify position/radius/weight
- Delete: Remove from storage and grids
- Query: Find neighbors, range search, field influence

## Token Lifecycle

1. **Creation:** Assign unique ID, validate position
2. **Active:** Can be queried, moved, updated
3. **Deletion:** Remove from storage (ID not reused)

## Relationships

**Computed Dynamically:**
- Neighbors: Tokens within radius distance
- Field influence: Sum of weighted influences
- Density: Token count in region

**Benefits:**
- No stale connections
- Always reflects current positions
- Efficient with caching

## Memory Overhead

**Per token:** ~100-150 bytes
- Position: 64 bytes (8 × f64)
- Metadata: 40-80 bytes (ID, radius, weight, timestamps)

**Scalability:** 1M tokens = ~100-150 MB

## References

- Cognitive Architecture Literature
- Spatial Computing Patterns
- NeuroGraph Core Implementation

## Revision History

- 2024-11-20: Initial decision
- 2026-01-09: Production validation
