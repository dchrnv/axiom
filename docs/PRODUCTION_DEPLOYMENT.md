# NeuroGraph - Production Deployment Guide

**Version:** v0.67.2
**Date:** 2026-01-11
**Target:** Production environments (Docker, Kubernetes, VM)

---

## Table of Contents

1. [Overview](#overview)
2. [System Requirements](#system-requirements)
3. [Quick Start (Docker Compose)](#quick-start-docker-compose)
4. [Production Configuration](#production-configuration)
5. [Environment Variables](#environment-variables)
6. [Deployment Methods](#deployment-methods)
7. [Security Hardening](#security-hardening)
8. [Monitoring & Observability](#monitoring--observability)
9. [Scaling & Performance](#scaling--performance)
10. [Backup & Recovery](#backup--recovery)
11. [Troubleshooting](#troubleshooting)

---

## Overview

NeuroGraph is a high-performance cognitive platform combining Rust Core, FastAPI, and WebSocket real-time communication. This guide covers production deployment best practices.

### Architecture

```
┌─────────────────────────────────────────────┐
│          Load Balancer / Ingress            │
└─────────────────┬───────────────────────────┘
                  │
        ┌─────────┴─────────┐
        │                   │
┌───────▼────────┐  ┌──────▼────────┐
│  API Server 1  │  │  API Server 2 │
│   (FastAPI)    │  │   (FastAPI)   │
└───────┬────────┘  └──────┬────────┘
        │                   │
        └─────────┬─────────┘
                  │
       ┌──────────┴──────────┐
       │                     │
┌──────▼──────┐  ┌──────────▼────────┐
│  Prometheus │  │     Grafana       │
│  (Metrics)  │  │  (Visualization)  │
└─────────────┘  └───────────────────┘
```

**Key Components:**
- **API Server**: FastAPI application with Rust Core integration
- **Prometheus**: Metrics collection and alerting
- **Grafana**: Visualization dashboards
- **Jaeger** (optional): Distributed tracing

---

## System Requirements

### Minimum Requirements

| Component | Requirement |
|-----------|-------------|
| CPU | 2 cores @ 2.0 GHz |
| RAM | 2 GB |
| Disk | 10 GB SSD |
| OS | Linux (Ubuntu 20.04+, RHEL 8+, Debian 11+) |
| Docker | 20.10+ |
| Docker Compose | 2.0+ |

### Recommended for Production

| Component | Requirement |
|-----------|-------------|
| CPU | 4 cores @ 2.5 GHz |
| RAM | 8 GB |
| Disk | 50 GB SSD (NVMe preferred) |
| Network | 1 Gbps |
| Load Balancer | NGINX, HAProxy, or cloud LB |

### Performance Capacity

- **Throughput**: 10K requests/sec per instance
- **WebSocket**: 1K+ concurrent connections per instance
- **Tokens**: 100M tokens sustained (3.7M tokens/s)
- **Latency**: p95 < 100ms, p99 < 200ms

---

## Quick Start (Docker Compose)

### 1. Clone Repository

```bash
git clone https://github.com/dchrnv/neurograph.git
cd neurograph
```

### 2. Create Environment File

```bash
cp .env.production.example .env.production
```

Edit `.env.production` with your configuration:

```bash
# Required secrets
SECRET_KEY=your-secret-key-here-min-32-chars
JWT_SECRET_KEY=your-jwt-secret-key-min-32-chars
GRAFANA_ADMIN_PASSWORD=strong-password-here
GRAFANA_SECRET_KEY=grafana-secret-key-min-32-chars

# API configuration
API_PORT=8000
API_WORKERS=4
NEUROGRAPH_MAX_TOKENS=100000
NEUROGRAPH_MAX_MEMORY_BYTES=2147483648

# Monitoring
PROMETHEUS_PORT=9090
GRAFANA_PORT=3000
```

### 3. Generate Secrets

```bash
# Generate SECRET_KEY
openssl rand -hex 32

# Generate JWT_SECRET_KEY
openssl rand -hex 32

# Generate GRAFANA_SECRET_KEY
openssl rand -hex 32
```

### 4. Deploy Stack

```bash
# Start production stack
docker-compose -f config/docker/docker-compose.production.yml up -d

# Check status
docker-compose -f config/docker/docker-compose.production.yml ps

# View logs
docker-compose -f config/docker/docker-compose.production.yml logs -f neurograph-api
```

### 5. Verify Deployment

```bash
# Health check
curl http://localhost:8000/api/v1/health

# Metrics
curl http://localhost:8000/metrics

# Grafana UI
open http://localhost:3000
```

---

## Production Configuration

### Docker Image Sizes

| Image | Size | Optimization |
|-------|------|--------------|
| `neurograph/api` | <300MB | Multi-stage build, Python slim |
| `prom/prometheus` | ~250MB | Official image |
| `grafana/grafana` | ~350MB | Official image |
| **Total** | ~900MB | Optimized for production |

### Resource Allocation

**API Server:**
```yaml
deploy:
  resources:
    limits:
      cpus: '2.0'
      memory: 2G
    reservations:
      cpus: '0.5'
      memory: 512M
```

**Prometheus:**
```yaml
deploy:
  resources:
    limits:
      cpus: '1.0'
      memory: 1G
    reservations:
      cpus: '0.25'
      memory: 256M
```

**Grafana:**
```yaml
deploy:
  resources:
    limits:
      cpus: '0.5'
      memory: 512M
    reservations:
      cpus: '0.1'
      memory: 128M
```

### Health Checks

All services have configured health checks:

**API Server:**
```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:8000/api/v1/health/live"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 40s
```

**Prometheus:**
```yaml
healthcheck:
  test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:9090/-/healthy"]
  interval: 30s
  timeout: 10s
  retries: 3
```

**Grafana:**
```yaml
healthcheck:
  test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:3000/api/health"]
  interval: 30s
  timeout: 10s
  retries: 3
```

---

## Environment Variables

### Required Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `SECRET_KEY` | Application secret key (min 32 chars) | `openssl rand -hex 32` |
| `JWT_SECRET_KEY` | JWT signing key (min 32 chars) | `openssl rand -hex 32` |
| `GRAFANA_ADMIN_PASSWORD` | Grafana admin password | Strong password |
| `GRAFANA_SECRET_KEY` | Grafana secret key | `openssl rand -hex 32` |

### Optional Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `API_PORT` | 8000 | API server port |
| `API_WORKERS` | 4 | Uvicorn worker count |
| `LOG_LEVEL` | info | Logging level (debug/info/warning/error) |
| `CORS_ORIGINS` | `["*"]` | Allowed CORS origins |
| `NEUROGRAPH_MAX_TOKENS` | 100000 | Max tokens in storage |
| `NEUROGRAPH_MAX_MEMORY_BYTES` | 2147483648 | Max memory usage (2GB) |
| `PROMETHEUS_RETENTION` | 15d | Prometheus data retention |
| `PROMETHEUS_PORT` | 9090 | Prometheus port |
| `GRAFANA_PORT` | 3000 | Grafana port |

### Performance Tuning

| Variable | Default | Description |
|----------|---------|-------------|
| `MAX_REQUESTS` | 10000 | Requests before worker restart |
| `MAX_REQUESTS_JITTER` | 1000 | Random jitter for worker restart |
| `API_CPU_LIMIT` | 2.0 | CPU limit (cores) |
| `API_MEMORY_LIMIT` | 2G | Memory limit |
| `STORAGE_BACKEND` | memory | Storage backend (memory/runtime) |

---

## Deployment Methods

### Method 1: Docker Compose (Recommended for Small-Medium Scale)

**Pros:**
- Simple setup and management
- Built-in networking and volumes
- Easy local development parity

**Cons:**
- Limited to single host
- Manual scaling

**Usage:**
```bash
docker-compose -f config/docker/docker-compose.production.yml up -d
```

### Method 2: Kubernetes (Recommended for Large Scale)

**Pros:**
- Automatic scaling (HPA)
- High availability
- Rolling updates
- Service discovery

**Cons:**
- Complex setup
- Requires K8s knowledge

**Example Deployment:**

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: neurograph-api
  namespace: neurograph
spec:
  replicas: 3
  selector:
    matchLabels:
      app: neurograph-api
  template:
    metadata:
      labels:
        app: neurograph-api
    spec:
      containers:
      - name: api
        image: neurograph/api:v0.67.2
        ports:
        - containerPort: 8000
        env:
        - name: SECRET_KEY
          valueFrom:
            secretKeyRef:
              name: neurograph-secrets
              key: secret-key
        - name: JWT_SECRET_KEY
          valueFrom:
            secretKeyRef:
              name: neurograph-secrets
              key: jwt-secret-key
        livenessProbe:
          httpGet:
            path: /api/v1/health/live
            port: 8000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/v1/health/ready
            port: 8000
          initialDelaySeconds: 10
          periodSeconds: 5
        resources:
          requests:
            cpu: 500m
            memory: 512Mi
          limits:
            cpu: 2000m
            memory: 2Gi
---
apiVersion: v1
kind: Service
metadata:
  name: neurograph-api
  namespace: neurograph
spec:
  selector:
    app: neurograph-api
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8000
  type: LoadBalancer
```

**Deploy:**
```bash
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
kubectl apply -f k8s/ingress.yaml
```

### Method 3: VM/Bare Metal

**Installation:**

```bash
# 1. Install Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER

# 2. Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.23.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# 3. Clone and deploy
git clone https://github.com/dchrnv/neurograph.git
cd neurograph
cp .env.production.example .env.production
# Edit .env.production with your secrets
docker-compose -f config/docker/docker-compose.production.yml up -d
```

---

## Security Hardening

### 1. Secrets Management

**DO NOT** hardcode secrets in code or config files.

**Best Practices:**
- Use environment variables
- Store secrets in secret managers (AWS Secrets Manager, HashiCorp Vault, etc.)
- Rotate secrets regularly (every 90 days)
- Use strong, randomly generated secrets (min 32 characters)

**Example with Docker Secrets:**
```bash
# Create secret
echo "your-secret-key" | docker secret create neurograph_secret_key -

# Use in docker-compose
services:
  neurograph-api:
    secrets:
      - neurograph_secret_key
    environment:
      - SECRET_KEY_FILE=/run/secrets/neurograph_secret_key

secrets:
  neurograph_secret_key:
    external: true
```

### 2. Network Security

**Firewall Rules:**
```bash
# Allow only necessary ports
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable
```

**Docker Network Isolation:**
- Use separate networks for API and monitoring
- Enable `internal: true` for monitoring network if not exposing externally
- Use `--network=host` only when necessary

### 3. HTTPS/TLS Configuration

**NGINX Reverse Proxy:**

```nginx
server {
    listen 443 ssl http2;
    server_name api.neurograph.example.com;

    ssl_certificate /etc/nginx/ssl/fullchain.pem;
    ssl_certificate_key /etc/nginx/ssl/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    location / {
        proxy_pass http://localhost:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # WebSocket support
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

### 4. User Permissions

All containers run as non-root users:
- API: `neurograph` (UID 1000)
- Prometheus: `nobody` (UID 65534)
- Grafana: `grafana` (UID 472)

### 5. Rate Limiting

API has built-in rate limiting:
- Per-IP: 100 requests/minute
- Per-user: 1000 requests/minute
- WebSocket: 10 connections per IP

---

## Monitoring & Observability

### Health Endpoints

| Endpoint | Purpose | Use Case |
|----------|---------|----------|
| `/api/v1/health` | Full health check | Manual verification |
| `/api/v1/health/live` | Liveness probe | K8s liveness |
| `/api/v1/health/ready` | Readiness probe | K8s readiness |
| `/api/v1/health/startup` | Startup probe | K8s startup |
| `/api/v1/health/components` | Component details | Debugging |

### Metrics

Access Prometheus metrics at: `http://localhost:8000/metrics`

**Key Metrics:**
- `http_requests_total` - Total HTTP requests
- `http_request_duration_seconds` - Request latency
- `process_cpu_seconds_total` - CPU usage
- `process_resident_memory_bytes` - Memory usage
- `websocket_connections_active` - Active WebSocket connections
- `token_storage_count` - Total tokens in storage

### Grafana Dashboards

Access at: `http://localhost:3000`

**Available Dashboards:**
1. **System Overview** - CPU, memory, request rate, latency
2. **WebSocket Metrics** - Connections, messages, channels
3. **Token & Grid Metrics** - Token operations, grid queries

### Prometheus Alerts

**9 configured alerts:**
- HighErrorRate (>5% 5xx responses)
- HighLatency (p95 > 1s)
- HighMemoryUsage (>1.5GB)
- HighCPUUsage (>80%)
- WebSocketConnectionsDrop
- ServiceDown
- UnusuallyHighRequestRate (>1000 req/s)
- HighTokenCount (>90k)
- HighBufferedEvents (>500)

View alerts at: `http://localhost:9090/alerts`

### Structured Logging

All logs are JSON-formatted for easy parsing:

```json
{
  "timestamp": "2026-01-11T12:00:00Z",
  "level": "INFO",
  "component": "api",
  "message": "Request processed",
  "request_id": "abc123",
  "duration_ms": 45,
  "status_code": 200
}
```

**Log Aggregation:**
- Use ELK stack (Elasticsearch, Logstash, Kibana)
- Or Loki + Grafana
- Or cloud solutions (CloudWatch, Stackdriver)

---

## Scaling & Performance

### Horizontal Scaling

**Docker Compose:**
```bash
docker-compose -f config/docker/docker-compose.production.yml up -d --scale neurograph-api=3
```

**Kubernetes:**
```bash
kubectl scale deployment neurograph-api --replicas=5
```

**Auto-scaling (HPA):**
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: neurograph-api-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: neurograph-api
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

### Load Balancing

**NGINX:**
```nginx
upstream neurograph_api {
    least_conn;
    server 10.0.1.10:8000 max_fails=3 fail_timeout=30s;
    server 10.0.1.11:8000 max_fails=3 fail_timeout=30s;
    server 10.0.1.12:8000 max_fails=3 fail_timeout=30s;
}

server {
    listen 80;
    server_name api.neurograph.example.com;

    location / {
        proxy_pass http://neurograph_api;
        proxy_next_upstream error timeout http_502 http_503 http_504;
    }
}
```

### Performance Tuning

**API Workers:**
- Rule of thumb: `(2 x CPU cores) + 1`
- For 4 cores: `API_WORKERS=9`

**Memory:**
- Min: 512MB per worker
- Recommended: 1GB per worker
- For 4 workers: 4GB total

**Database Connection Pool:**
```python
# Adjust in config
MAX_CONNECTIONS = API_WORKERS * 5
```

---

## Backup & Recovery

### Data to Backup

1. **Prometheus Data** - `/prometheus` volume
2. **Grafana Data** - `/var/lib/grafana` volume
3. **API Data** - `/app/data` volume
4. **API Logs** - `/app/logs` volume

### Backup Script

```bash
#!/bin/bash
# backup.sh

BACKUP_DIR="/backups/neurograph"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p "$BACKUP_DIR/$DATE"

# Backup Prometheus data
docker run --rm \
  -v neurograph_prometheus-data:/data \
  -v "$BACKUP_DIR/$DATE":/backup \
  alpine tar czf /backup/prometheus.tar.gz -C /data .

# Backup Grafana data
docker run --rm \
  -v neurograph_grafana-data:/data \
  -v "$BACKUP_DIR/$DATE":/backup \
  alpine tar czf /backup/grafana.tar.gz -C /data .

# Backup API data
docker run --rm \
  -v neurograph_api-data:/data \
  -v "$BACKUP_DIR/$DATE":/backup \
  alpine tar czf /backup/api-data.tar.gz -C /data .

echo "Backup completed: $BACKUP_DIR/$DATE"
```

### Restore Script

```bash
#!/bin/bash
# restore.sh

BACKUP_DATE="20260111_120000"
BACKUP_DIR="/backups/neurograph/$BACKUP_DATE"

# Stop services
docker-compose -f config/docker/docker-compose.production.yml down

# Restore Prometheus
docker run --rm \
  -v neurograph_prometheus-data:/data \
  -v "$BACKUP_DIR":/backup \
  alpine sh -c "cd /data && tar xzf /backup/prometheus.tar.gz"

# Restore Grafana
docker run --rm \
  -v neurograph_grafana-data:/data \
  -v "$BACKUP_DIR":/backup \
  alpine sh -c "cd /data && tar xzf /backup/grafana.tar.gz"

# Restore API data
docker run --rm \
  -v neurograph_api-data:/data \
  -v "$BACKUP_DIR":/backup \
  alpine sh -c "cd /data && tar xzf /backup/api-data.tar.gz"

# Start services
docker-compose -f config/docker/docker-compose.production.yml up -d

echo "Restore completed from: $BACKUP_DIR"
```

### Automated Backups

```bash
# Add to crontab
crontab -e

# Daily backup at 2 AM
0 2 * * * /path/to/backup.sh

# Weekly cleanup (keep last 4 weeks)
0 3 * * 0 find /backups/neurograph -type d -mtime +28 -exec rm -rf {} \;
```

---

## Troubleshooting

### Common Issues

#### 1. API Server Won't Start

**Symptoms:**
- Container exits immediately
- Health check fails

**Diagnosis:**
```bash
docker-compose logs neurograph-api
```

**Common Causes:**
- Missing environment variables
- Port already in use
- Insufficient memory

**Solutions:**
```bash
# Check environment variables
docker-compose config

# Check port usage
netstat -tuln | grep 8000

# Increase memory limit in docker-compose.yml
```

#### 2. High Memory Usage

**Symptoms:**
- OOM kills
- Slow performance

**Diagnosis:**
```bash
docker stats neurograph-api
```

**Solutions:**
```bash
# Reduce max tokens
NEUROGRAPH_MAX_TOKENS=50000

# Add more memory
deploy:
  resources:
    limits:
      memory: 4G
```

#### 3. Prometheus Alerts Not Firing

**Diagnosis:**
```bash
# Check Prometheus targets
curl http://localhost:9090/api/v1/targets

# Check alert rules
curl http://localhost:9090/api/v1/rules
```

**Solutions:**
```bash
# Reload Prometheus config
curl -X POST http://localhost:9090/-/reload

# Check alert rules syntax
promtool check rules prometheus-alerts.yml
```

#### 4. Grafana Dashboards Not Loading

**Diagnosis:**
```bash
docker-compose logs grafana
```

**Solutions:**
```bash
# Check datasource connection
curl http://localhost:3000/api/datasources

# Restart Grafana
docker-compose restart grafana
```

### Performance Issues

#### Slow Response Times

**Check:**
1. CPU/Memory usage: `docker stats`
2. Request latency: Check Grafana "API Performance" dashboard
3. Database queries: Enable query logging

**Optimize:**
```bash
# Increase workers
API_WORKERS=8

# Enable caching
ENABLE_CACHE=true
CACHE_TTL=300
```

#### WebSocket Connection Drops

**Check:**
1. Connection count: `/metrics` → `websocket_connections_active`
2. Buffered events: `/metrics` → `websocket_buffered_events`
3. Network issues: `docker-compose logs neurograph-api | grep websocket`

**Optimize:**
```bash
# Increase connection limit
MAX_WEBSOCKET_CONNECTIONS=2000

# Increase buffer size
WEBSOCKET_BUFFER_SIZE=1000
```

---

## Appendix

### A. Complete .env.production Template

See: `.env.production.example`

### B. Kubernetes Manifests

See: `k8s/` directory

### C. NGINX Configuration

See: `examples/nginx/` directory

### D. Monitoring Setup

See: `docs/PHASE_3.1_MONITORING_COMPLETE.md`

### E. Performance Benchmarks

See: `docs/PERFORMANCE_SUMMARY.md`

---

## Support

- **Documentation**: https://github.com/dchrnv/neurograph
- **Issues**: https://github.com/dchrnv/neurograph/issues
- **PyPI**: https://pypi.org/project/ngcore/

---

**Version:** v0.67.2
**Last Updated:** 2026-01-11
**Status:** Production Ready ✅
