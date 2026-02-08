Architecture
============

NeuroGraph system architecture overview.

System Overview
---------------

NeuroGraph consists of three main layers:

1. **Rust Core** (``src/core/``)
   - Spatial algorithms
   - Grid operations
   - Performance-critical code
   - Exposed via FFI

2. **Python API** (``src/api/``)
   - FastAPI REST endpoints
   - WebSocket server
   - Authentication & authorization
   - Prometheus metrics

3. **Storage Layer** (``src/api/storage/``)
   - In-memory storage
   - Token management
   - Grid management
   - CDNA configuration

Components
----------

Token System
~~~~~~~~~~~~

Tokens are 8-dimensional spatial entities with:

- **Position**: 8D coordinates
- **Radius**: Influence radius
- **Weight**: Importance factor
- **Connections**: Links to other tokens

Grid System
~~~~~~~~~~~

Spatial indexing with:

- **Cell-based partitioning**: O(1) lookups
- **Neighbor finding**: Efficient radius queries
- **Field influence**: Calculate spatial fields
- **Density maps**: Token distribution analysis

CDNA (Cognitive DNA)
~~~~~~~~~~~~~~~~~~~~

Dynamic configuration system:

- **Profiles**: Pre-defined behavior patterns
- **Dimension scales**: Per-dimension multipliers
- **Quarantine mode**: Safe testing environment
- **History tracking**: Configuration audit log

WebSocket Layer
~~~~~~~~~~~~~~~

Real-time event broadcasting:

- **Channels**: tokens, grid, cdna
- **Subscriptions**: Per-client channel management
- **Event buffering**: Prevent message loss
- **Metrics**: Connection tracking

Authentication
~~~~~~~~~~~~~~

JWT-based security:

- **Access tokens**: Short-lived (15 min)
- **Refresh tokens**: Long-lived (7 days)
- **Permissions**: Fine-grained scopes
- **API keys**: Machine-to-machine auth

Data Flow
---------

Request Flow
~~~~~~~~~~~~

.. code-block:: text

   Client Request
       ↓
   FastAPI Router
       ↓
   Authentication Middleware
       ↓
   Permission Check
       ↓
   Business Logic
       ↓
   Storage Layer
       ↓
   Rust Core (if needed)
       ↓
   Response + WebSocket Broadcast

WebSocket Flow
~~~~~~~~~~~~~~

.. code-block:: text

   WebSocket Client
       ↓
   Connection Manager
       ↓
   Channel Subscription
       ↓
   Event Listener
       ↓
   Message Broadcast
       ↓
   All Subscribed Clients

Performance
-----------

Rust FFI
~~~~~~~~

- 50-100x speedup for spatial operations
- Zero-copy data transfer where possible
- Connection caching (~50x improvement)

Optimizations
~~~~~~~~~~~~~

- Connection result caching
- Grid cell-based indexing
- Efficient neighbor search
- Prometheus metric sampling

Monitoring
----------

Health Checks
~~~~~~~~~~~~~

- **Liveness**: ``/api/v1/health/live``
- **Readiness**: ``/api/v1/health/ready``
- **Startup**: ``/api/v1/health/startup``

Metrics
~~~~~~~

Prometheus metrics at ``/metrics``:

- Request counts and latencies
- WebSocket connections
- Token/grid operations
- Error rates

See :doc:`api/websocket` for detailed metrics.
