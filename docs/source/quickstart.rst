Quick Start
===========

Get Axiom up and running in 5 minutes.

Prerequisites
-------------

- Python 3.11+
- Rust 1.70+
- Git

Installation
------------

1. **Clone the repository**:

   .. code-block:: bash

      git clone https://github.com/chrnv/axiom-os-mvp.git
      cd axiom-os-mvp

2. **Install Python dependencies**:

   .. code-block:: bash

      pip install -r requirements.txt

3. **Build Rust core**:

   .. code-block:: bash

      cd src/core
      cargo build --release
      cd ../..

4. **Run the server**:

   .. code-block:: bash

      ./run.sh

5. **Verify it's working**:

   .. code-block:: bash

      curl http://localhost:8000/api/v1/health

First Steps
-----------

Authentication
~~~~~~~~~~~~~~

Get an access token:

.. code-block:: bash

   curl -X POST http://localhost:8000/api/v1/auth/login \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "admin"}'

Create a Token
~~~~~~~~~~~~~~

.. code-block:: bash

   curl -X POST http://localhost:8000/api/v1/tokens \
     -H "Authorization: Bearer <your_token>" \
     -H "Content-Type: application/json" \
     -d '{
       "position": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
       "radius": 1.0,
       "weight": 1.0
     }'

Create a Grid
~~~~~~~~~~~~~

.. code-block:: bash

   curl -X POST http://localhost:8000/api/v1/grid \
     -H "Authorization: Bearer <your_token>" \
     -H "Content-Type: application/json" \
     -d '{
       "cell_size": 2.0,
       "dimensions": 8
     }'

WebSocket Connection
~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

   import asyncio
   import websockets
   import json

   async def listen():
       uri = "ws://localhost:8000/ws"
       async with websockets.connect(uri) as websocket:
           # Subscribe to tokens channel
           await websocket.send(json.dumps({
               "type": "subscribe",
               "channel": "tokens"
           }))

           # Listen for events
           async for message in websocket:
               data = json.loads(message)
               print(f"Received: {data}")

   asyncio.run(listen())

Next Steps
----------

- Read the :doc:`architecture` overview
- Explore the :doc:`api/index`
- Check out example notebooks in ``docs/jupyter/``
