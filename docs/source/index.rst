Axiom Documentation
========================

**Axiom** is a high-performance spatial computing system based on tokens, combining:

- **Rust Core**: Ultra-fast spatial algorithms with 50-100x performance improvements
- **Python API**: Production-ready FastAPI backend with JWT auth, WebSocket support, and Prometheus metrics
- **Dynamic CDNA**: Cognitive DNA profiles for adaptive spatial behavior

Version: |version|

Features
--------

🚀 **Performance**
   - Rust-powered spatial operations (FFI)
   - Connection caching with ~50x speedup
   - Optimized grid queries

🔐 **Security**
   - JWT authentication (access + refresh tokens)
   - Fine-grained permissions system
   - API key management
   - Rate limiting

📊 **Observability**
   - Prometheus metrics
   - Health checks (liveness, readiness, startup)
   - WebSocket connection tracking

🧬 **CDNA System**
   - Multiple cognitive profiles
   - Quarantine mode for safe testing
   - Dynamic dimension scaling
   - Configuration history

🔌 **Real-time**
   - WebSocket channels (tokens, grid, cdna)
   - Event buffering
   - Broadcast optimizations

Quick Links
-----------

- :doc:`quickstart` - Get started in 5 minutes
- :doc:`api/index` - Complete API reference
- `GitHub Repository <https://github.com/chrnv/axiom-os-mvp>`_

Getting Started
---------------

.. code-block:: bash

   # 1. Install dependencies
   pip install -r requirements.txt
   cargo build --release

   # 2. Run the server
   ./run.sh

   # 3. Access the API
   curl http://localhost:8000/api/v1/health

Documentation
-------------

.. toctree::
   :maxdepth: 2
   :caption: User Guide

   quickstart
   architecture
   configuration

.. toctree::
   :maxdepth: 2
   :caption: API Reference

   api/index
   api/storage
   api/auth
   api/websocket
   api/models
   api/main

.. toctree::
   :maxdepth: 1
   :caption: Development

   contributing
   testing
   changelog

Indices and Tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`

