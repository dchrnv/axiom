API Reference
=============

Complete Python API reference for NeuroGraph.

Overview
--------

The NeuroGraph API is organized into the following modules:

- **Storage Layer** (:doc:`storage`) - Token, grid, and CDNA storage implementations
- **Authentication** (:doc:`auth`) - JWT auth and permissions system
- **WebSocket** (:doc:`websocket`) - Real-time communication and metrics
- **Data Models** (:doc:`models`) - Pydantic request/response models
- **Main Application** (:doc:`main`) - FastAPI app and startup handlers

Module Index
------------

.. toctree::
   :maxdepth: 2

   storage
   auth
   websocket
   models
   main

REST API
--------

The OpenAPI/Swagger documentation is available at:

- **Interactive Docs**: http://localhost:8000/docs
- **OpenAPI Schema**: http://localhost:8000/openapi.json

Key Endpoints
~~~~~~~~~~~~~

**Authentication**
   - ``POST /api/v1/auth/login`` - User login
   - ``POST /api/v1/auth/refresh`` - Refresh access token
   - ``POST /api/v1/auth/logout`` - Logout

**Tokens**
   - ``GET /api/v1/tokens`` - List all tokens
   - ``POST /api/v1/tokens`` - Create token
   - ``GET /api/v1/tokens/{id}`` - Get token by ID
   - ``PUT /api/v1/tokens/{id}`` - Update token
   - ``DELETE /api/v1/tokens/{id}`` - Delete token

**Grid**
   - ``POST /api/v1/grid`` - Create grid
   - ``GET /api/v1/grid/{id}`` - Get grid info
   - ``GET /api/v1/grid/{id}/neighbors/{token_id}`` - Find neighbors
   - ``POST /api/v1/grid/{id}/range`` - Range query

**CDNA**
   - ``GET /api/v1/cdna`` - Get current configuration
   - ``PUT /api/v1/cdna`` - Update configuration
   - ``GET /api/v1/cdna/profiles`` - List profiles
   - ``POST /api/v1/cdna/profiles/{profile_id}`` - Switch profile
   - ``POST /api/v1/cdna/quarantine/start`` - Start quarantine mode
   - ``POST /api/v1/cdna/quarantine/stop`` - Stop quarantine mode

**Monitoring**
   - ``GET /api/v1/health`` - Health check
   - ``GET /api/v1/health/live`` - Liveness probe
   - ``GET /api/v1/health/ready`` - Readiness probe
   - ``GET /metrics`` - Prometheus metrics

WebSocket API
-------------

Connect to ``ws://localhost:8000/ws``

**Message Format**:

.. code-block:: json

   {
     "type": "subscribe",
     "channel": "tokens"
   }

**Channels**:
   - ``tokens`` - Token creation/updates/deletions
   - ``grid`` - Grid structure changes
   - ``cdna`` - CDNA configuration changes

**Message Types**:
   - ``subscribe`` - Subscribe to channel
   - ``unsubscribe`` - Unsubscribe from channel
   - ``ping`` - Heartbeat (receives ``pong``)
