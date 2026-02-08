Configuration
=============

NeuroGraph configuration guide.

Environment Variables
---------------------

Core Settings
~~~~~~~~~~~~~

.. code-block:: bash

   # Server
   HOST=0.0.0.0
   PORT=8000
   WORKERS=4

   # Logging
   LOG_LEVEL=INFO
   LOG_FORMAT=json

   # CORS
   CORS_ORIGINS=http://localhost:3000,http://localhost:8080

Authentication
~~~~~~~~~~~~~~

.. code-block:: bash

   # JWT Settings
   SECRET_KEY=your-secret-key-here
   ACCESS_TOKEN_EXPIRE_MINUTES=15
   REFRESH_TOKEN_EXPIRE_DAYS=7

   # Default Admin
   ADMIN_USERNAME=admin
   ADMIN_PASSWORD=changeme

Storage
~~~~~~~

.. code-block:: bash

   # Memory limits
   MAX_TOKENS=100000
   MAX_GRIDS=10

   # Grid defaults
   DEFAULT_CELL_SIZE=2.0
   DEFAULT_DIMENSIONS=8

CDNA Profiles
-------------

Profiles are defined in ``src/core/cdna/profiles.py``.

Available Profiles
~~~~~~~~~~~~~~~~~~

**explorer**
   - High plasticity (0.8)
   - Moderate evolution rate (0.5)
   - Balanced scales: [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0]

**creative**
   - Very high plasticity (0.9)
   - High evolution rate (0.7)
   - Experimental scales: [2.0, 3.0, 2.5, 4.0, 5.0, 4.0, 3.0, 8.0]

**conservative**
   - Low plasticity (0.3)
   - Low evolution rate (0.2)
   - Restricted scales: [1.0, 1.0, 1.0, 1.5, 2.0, 1.5, 1.0, 2.5]

Custom Profile
~~~~~~~~~~~~~~

Create a custom profile:

.. code-block:: python

   from api.storage.memory import get_cdna_storage

   storage = get_cdna_storage()

   # Define custom scales
   custom_scales = [1.2, 1.8, 1.5, 2.5, 3.5, 3.0, 2.5, 6.0]

   # Apply (will be validated)
   storage.update_config(
       profile="custom",
       dimension_scales=custom_scales
   )

Docker Configuration
--------------------

The project includes Docker support via ``docker-compose.yml``.

Building
~~~~~~~~

.. code-block:: bash

   docker-compose build

Running
~~~~~~~

.. code-block:: bash

   docker-compose up -d

Environment Override
~~~~~~~~~~~~~~~~~~~~

Create a ``.env`` file:

.. code-block:: bash

   # .env
   SECRET_KEY=production-secret-key
   LOG_LEVEL=WARNING
   ADMIN_PASSWORD=secure-password

Prometheus Integration
----------------------

Metrics Configuration
~~~~~~~~~~~~~~~~~~~~~

Edit ``prometheus.yml``:

.. code-block:: yaml

   scrape_configs:
     - job_name: 'neurograph'
       static_configs:
         - targets: ['localhost:8000']
       metrics_path: '/metrics'
       scrape_interval: 15s

Grafana Dashboard
~~~~~~~~~~~~~~~~~

Import the dashboard from ``docs/monitoring/grafana-dashboard.json``.

Key metrics to monitor:

- ``neurograph_http_requests_total``
- ``neurograph_ws_connections_total``
- ``neurograph_tokens_total``
- ``neurograph_grid_operations_duration_seconds``

Logging
-------

Log Format
~~~~~~~~~~

JSON format (default in production):

.. code-block:: json

   {
     "timestamp": "2026-01-09T18:00:00Z",
     "level": "INFO",
     "logger": "api.main",
     "message": "Server started",
     "context": {
       "host": "0.0.0.0",
       "port": 8000
     }
   }

Log Levels
~~~~~~~~~~

- ``DEBUG``: Detailed debugging information
- ``INFO``: General informational messages
- ``WARNING``: Warning messages
- ``ERROR``: Error messages
- ``CRITICAL``: Critical errors

Production Recommendations
--------------------------

Security
~~~~~~~~

1. Change default admin credentials
2. Use strong ``SECRET_KEY`` (32+ random bytes)
3. Enable HTTPS (reverse proxy with nginx/traefik)
4. Restrict CORS origins
5. Use API keys for machine-to-machine communication

Performance
~~~~~~~~~~~

1. Set appropriate ``WORKERS`` (2-4 per CPU core)
2. Enable connection caching (default: on)
3. Configure rate limiting
4. Monitor memory usage

Monitoring
~~~~~~~~~~

1. Set up Prometheus scraping
2. Import Grafana dashboards
3. Configure alerts (high error rate, memory usage)
4. Enable health check probes

High Availability
~~~~~~~~~~~~~~~~~

1. Run multiple instances behind load balancer
2. Use Redis for shared session state (future)
3. Implement graceful shutdown
4. Configure readiness/liveness probes
