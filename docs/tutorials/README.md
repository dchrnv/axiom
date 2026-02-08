# NeuroGraph Tutorials

Comprehensive hands-on tutorials for mastering NeuroGraph.

## Overview

**7 tutorials** covering Basic, Intermediate, and Advanced topics
**Total time:** ~3-4 hours
**Format:** Interactive Jupyter notebooks

## Tutorial Path

### 📘 Basic Level (60-80 minutes)

Foundation concepts and operations:

1. **[Token Operations](basic/01_token_operations.ipynb)** (15-20 min)
   - CRUD operations for tokens
   - 8D coordinate system
   - Position, radius, weight attributes
   - Batch operations

2. **[Grid Queries](basic/02_grid_queries.ipynb)** (20-25 min)
   - Spatial indexing with grids
   - Neighbor search (radius queries)
   - Range queries (bounding box)
   - Field influence calculations
   - Density mapping
   - Visual examples with matplotlib

3. **[CDNA Profiles](basic/03_cdna_profiles.ipynb)** (15-20 min)
   - Understanding CDNA (Cognitive DNA)
   - Profile switching (explorer, creative, conservative)
   - Quarantine mode for safe testing
   - Custom profile creation
   - Configuration validation

### 📗 Intermediate Level (45-55 minutes)

Production integration patterns:

4. **[WebSocket Events](intermediate/01_websocket_events.ipynb)** (25-30 min)
   - Real-time event streaming
   - Channel subscriptions (tokens, grid, cdna)
   - Heartbeat mechanism
   - Error handling and reconnection
   - Event filtering
   - WebSocket best practices

5. **[REST API Integration](intermediate/02_rest_api_integration.ipynb)** (20-25 min)
   - Professional API client wrapper
   - Token auto-refresh strategies
   - Retry logic with exponential backoff
   - Rate limiting (token bucket)
   - Batch operations optimization
   - Error classification and recovery

### 📕 Advanced Level (60-80 minutes)

Performance and production:

6. **[Performance Optimization](advanced/01_performance_optimization.ipynb)** (30-40 min)
   - Benchmarking and profiling
   - Connection cache impact (~50x speedup)
   - Grid cell size optimization
   - Session reuse strategies
   - Memory profiling
   - Prometheus metrics analysis

7. **[Production Deployment](advanced/02_production_deployment.ipynb)** (30-40 min)
   - Docker multi-stage builds
   - Docker Compose stack
   - Kubernetes deployment
   - Health checks (liveness, readiness, startup)
   - Monitoring with Prometheus + Grafana
   - Security hardening checklist
   - Scaling strategies (horizontal/vertical)

## Prerequisites

### Software Requirements

- Python 3.11+
- Jupyter Lab or Jupyter Notebook
- NeuroGraph server running locally
- Optional: Docker, Kubernetes (for advanced tutorials)

### Python Packages

```bash
pip install jupyter requests websockets numpy matplotlib psutil
```

### Starting NeuroGraph

```bash
# Terminal 1: Start the server
./run.sh

# Terminal 2: Launch Jupyter
jupyter lab docs/tutorials/
```

## Running Tutorials

### Option 1: Jupyter Lab (Recommended)

```bash
cd docs/tutorials
jupyter lab
```

Navigate to the tutorial you want and run cells sequentially.

### Option 2: Jupyter Notebook

```bash
cd docs/tutorials
jupyter notebook
```

### Option 3: VS Code

Open the `.ipynb` files in VS Code with the Jupyter extension installed.

## Tutorial Structure

Each tutorial follows this format:

1. **Overview** - Learning objectives and prerequisites
2. **Setup** - Import dependencies and authenticate
3. **Step-by-step exercises** - Hands-on code examples
4. **Visualizations** - Charts and plots where applicable
5. **Summary** - Key takeaways
6. **Next steps** - Recommended follow-up tutorials

## Learning Path Recommendations

### For Beginners
Start with Basic tutorials 1-3 in order. Take breaks between tutorials to experiment with the concepts.

### For Developers
If you're experienced with APIs, you can skip Basic 1-2 and start with Basic 3 (CDNA), then move to Intermediate tutorials.

### For DevOps/SRE
Focus on:
- Intermediate 2 (REST API Integration)
- Advanced 1 (Performance Optimization)
- Advanced 2 (Production Deployment)

### For Data Scientists
Focus on:
- Basic 1-2 (Tokens and Grid Queries)
- Basic 3 (CDNA Profiles)
- Advanced 1 (Performance Optimization)

## Tips for Success

1. **Run code cells in order** - Each tutorial builds on previous cells
2. **Experiment freely** - Modify examples to test your understanding
3. **Clean up resources** - Run cleanup cells to avoid memory issues
4. **Check server logs** - Watch terminal output for real-time feedback
5. **Use visualization** - Plots help understand spatial relationships
6. **Take notes** - Document insights and questions
7. **Ask for help** - Join our Discord or open GitHub issues

## Common Issues

### Connection Refused
- **Cause:** NeuroGraph server not running
- **Fix:** Start server with `./run.sh`

### Authentication Failed
- **Cause:** Default credentials changed
- **Fix:** Update username/password in notebook

### Import Errors
- **Cause:** Missing Python packages
- **Fix:** `pip install jupyter requests websockets numpy matplotlib`

### Timeout Errors
- **Cause:** Long-running operations
- **Fix:** Increase timeout in code or run on more powerful machine

## Next Steps After Tutorials

1. **Explore the API Reference** - [docs/source/api/](../source/api/)
2. **Read Architecture Guide** - [docs/source/architecture.rst](../source/architecture.rst)
3. **Check Configuration Options** - [docs/source/configuration.rst](../source/configuration.rst)
4. **Build Your First Project** - Apply what you learned!

## Contributing

Found an issue or have a suggestion?

- **Bug reports:** [GitHub Issues](https://github.com/chrnv/neurograph-os-mvp/issues)
- **Improvements:** Submit a PR with fixes or new examples
- **New tutorials:** Propose ideas via GitHub Discussions

## Tutorial Statistics

- **Total notebooks:** 7
- **Total code cells:** ~200+
- **Total examples:** ~50+
- **Lines of code:** ~2000+
- **Visualizations:** 10+

## Changelog

### v0.65.0 (2026-01-09)
- ✅ Initial tutorial release
- 7 comprehensive notebooks
- Covers Basic to Advanced topics
- Interactive Jupyter format

---

**Happy Learning!** 🚀

For questions, check the [API Reference](../source/api/index.html) or [Quick Start Guide](../source/quickstart.html).
