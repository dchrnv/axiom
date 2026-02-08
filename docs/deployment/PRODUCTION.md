# NeuroGraph - Production Deployment Guide

**Version:** v0.67.0
**Last Updated:** 2026-01-10
**Target:** Production-ready deployment with Docker Compose

## Table of Contents

- [System Requirements](#system-requirements)
- [Pre-deployment Checklist](#pre-deployment-checklist)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Deployment Steps](#deployment-steps)
- [Monitoring & Observability](#monitoring--observability)
- [Security Hardening](#security-hardening)
- [Backup & Recovery](#backup--recovery)
- [Scaling](#scaling)
- [Troubleshooting](#troubleshooting)

## System Requirements

### Minimum Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **CPU** | 2 cores | 4+ cores |
| **RAM** | 4 GB | 8+ GB |
| **Disk** | 20 GB | 50+ GB SSD |
| **OS** | Linux (Ubuntu 20.04+) | Linux (Ubuntu 22.04+) |
| **Docker** | 24.0+ | Latest |
| **Docker Compose** | 2.20+ | Latest |

### Recommended Production Specs

- **CPU:** 4-8 cores (AMD EPYC / Intel Xeon)
- **RAM:** 16-32 GB
- **Disk:** 100-500 GB NVMe SSD
- **Network:** 1+ Gbps
- **OS:** Ubuntu 22.04 LTS / Debian 12

## Pre-deployment Checklist

### ✅ Infrastructure

- [ ] Server provisioned with recommended specs
- [ ] Docker & Docker Compose installed
- [ ] Firewall configured (ports 8000, 3000, 9090)
- [ ] SSL/TLS certificates obtained (Let's Encrypt)
- [ ] DNS records configured
- [ ] Backup storage configured

### ✅ Security

- [ ] Strong passwords generated for all services
- [ ] Secret keys generated (32+ byte random hex)
- [ ] SSH keys configured (password auth disabled)
- [ ] UFW/iptables firewall enabled
- [ ] Fail2ban installed and configured
- [ ] Regular security updates enabled

### ✅ Monitoring

- [ ] Log aggregation system ready (optional)
- [ ] Alert notification channels configured
- [ ] Uptime monitoring configured
- [ ] Backup verification automated

## Quick Start

### 1. Clone Repository

```bash
# SSH
git clone git@github.com:dchrnv/neurograph.git
cd neurograph

# HTTPS
git clone https://github.com/dchrnv/neurograph.git
cd neurograph
```

### 2. Configure Environment

```bash
# Copy example config
cp .env.production.example .env.production

# Generate secrets
openssl rand -hex 32  # SECRET_KEY
openssl rand -hex 32  # JWT_SECRET_KEY
openssl rand -hex 32  # GRAFANA_SECRET_KEY

# Edit configuration
nano .env.production
```

### 3. Create Data Directories

```bash
# Create persistent storage
mkdir -p data/{api,logs}
chmod 755 data
chmod 755 data/api data/logs
```

### 4. Deploy

```bash
# Build and start services
docker-compose -f config/docker/docker-compose.production.yml --env-file .env.production up -d

# Check status
docker-compose -f config/docker/docker-compose.production.yml ps

# View logs
docker-compose -f config/docker/docker-compose.production.yml logs -f
```

## Configuration

### Environment Variables

See `.env.production.example` for all available options.

#### Critical Settings

```bash
# Security - MUST CHANGE!
SECRET_KEY=<32-byte-hex>
JWT_SECRET_KEY=<32-byte-hex>
GRAFANA_ADMIN_PASSWORD=<strong-password>

# API
API_WORKERS=4              # Number of Uvicorn workers
API_PORT=8000              # API port

# Resource Limits
API_CPU_LIMIT=2.0          # CPU cores
API_MEMORY_LIMIT=2G        # RAM limit
```

#### Logging Configuration

```bash
# Production logging
LOG_LEVEL=info             # info | warning | error
LOG_JSON_FORMAT=true       # JSON structured logs
LOG_CORRELATION_TRACKING=true
```

#### Storage Configuration

```bash
# Storage backend
STORAGE_BACKEND=memory     # memory | runtime
NEUROGRAPH_MAX_TOKENS=100000
NEUROGRAPH_MAX_MEMORY_BYTES=2147483648  # 2GB
```

### Docker Compose Profiles

```bash
# Full stack (API + Monitoring)
docker-compose -f config/docker/docker-compose.production.yml up -d

# With distributed tracing
docker-compose -f config/docker/docker-compose.production.yml --profile tracing up -d
```

## Deployment Steps

### Step 1: Prepare Server

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER

# Install Docker Compose
sudo apt install docker-compose-plugin

# Verify installation
docker --version
docker compose version
```

### Step 2: Configure Firewall

```bash
# Install UFW
sudo apt install ufw -y

# Allow SSH
sudo ufw allow 22/tcp

# Allow HTTP/HTTPS (if using reverse proxy)
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Allow application ports
sudo ufw allow 8000/tcp   # API
sudo ufw allow 3000/tcp   # Grafana
sudo ufw allow 9090/tcp   # Prometheus

# Enable firewall
sudo ufw enable
```

### Step 3: Setup Reverse Proxy (Nginx)

```bash
# Install Nginx
sudo apt install nginx -y

# Configure site
sudo nano /etc/nginx/sites-available/neurograph
```

**Nginx Configuration:**

```nginx
server {
    listen 80;
    server_name your-domain.com;

    # Redirect to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name your-domain.com;

    # SSL certificates
    ssl_certificate /etc/letsencrypt/live/your-domain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/your-domain.com/privkey.pem;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # API proxy
    location /api/ {
        proxy_pass http://localhost:8000/api/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # WebSocket support
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }

    # Grafana proxy
    location /grafana/ {
        proxy_pass http://localhost:3000/;
        proxy_set_header Host $host;
    }
}
```

### Step 4: Deploy Application

```bash
# Navigate to project
cd /opt/neurograph

# Load environment
source .env.production

# Deploy
docker-compose -f config/docker/docker-compose.production.yml up -d

# Wait for services to start
sleep 30

# Check health
curl http://localhost:8000/api/v1/health
```

### Step 5: Verify Deployment

```bash
# Check service status
docker-compose -f config/docker/docker-compose.production.yml ps

# View logs
docker-compose -f config/docker/docker-compose.production.yml logs -f neurograph-api

# Test API
curl http://localhost:8000/api/v1/health/ready

# Test Prometheus
curl http://localhost:9090/-/healthy

# Test Grafana
curl http://localhost:3000/api/health
```

## Monitoring & Observability

### Access Dashboards

- **API Docs:** http://your-domain.com/api/v1/docs
- **Grafana:** http://your-domain.com:3000 (admin/your-password)
- **Prometheus:** http://your-domain.com:9090

### Key Metrics to Monitor

1. **System Health**
   - CPU usage < 80%
   - Memory usage < 80%
   - Disk usage < 85%

2. **Application Metrics**
   - Request rate (req/s)
   - Error rate < 1%
   - p95 latency < 500ms
   - Active WebSocket connections

3. **Alerts**
   - Service down
   - High error rate (>5%)
   - High latency (>1s)
   - Resource saturation

### Log Management

```bash
# View real-time logs
docker-compose -f config/docker/docker-compose.production.yml logs -f neurograph-api

# Search logs for errors
docker-compose -f config/docker/docker-compose.production.yml logs neurograph-api | grep ERROR

# Export logs
docker-compose -f config/docker/docker-compose.production.yml logs --no-color neurograph-api > api-logs.txt
```

## Security Hardening

### 1. SSL/TLS Setup

```bash
# Install Certbot
sudo apt install certbot python3-certbot-nginx -y

# Obtain certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal
sudo certbot renew --dry-run
```

### 2. Secure Docker Daemon

```bash
# Edit daemon.json
sudo nano /etc/docker/daemon.json
```

```json
{
  "live-restore": true,
  "userland-proxy": false,
  "no-new-privileges": true
}
```

### 3. Regular Updates

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Update Docker images
docker-compose -f config/docker/docker-compose.production.yml pull
docker-compose -f config/docker/docker-compose.production.yml up -d
```

### 4. Backup Strategy

```bash
# Backup script
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/backup/neurograph"

# Backup data
tar -czf $BACKUP_DIR/data_$DATE.tar.gz data/

# Backup configuration
cp .env.production $BACKUP_DIR/env_$DATE

# Backup Docker volumes
docker run --rm -v neurograph_api-data:/data -v $BACKUP_DIR:/backup \
  alpine tar -czf /backup/volumes_$DATE.tar.gz /data

# Cleanup old backups (keep 7 days)
find $BACKUP_DIR -name "*.tar.gz" -mtime +7 -delete
```

## Scaling

### Vertical Scaling

```bash
# Edit .env.production
API_WORKERS=8              # Increase workers
API_CPU_LIMIT=4.0          # Increase CPU
API_MEMORY_LIMIT=4G        # Increase RAM

# Restart
docker-compose -f config/docker/docker-compose.production.yml up -d
```

### Horizontal Scaling

For load-balanced deployment:

1. Deploy multiple API instances
2. Use Nginx/HAProxy for load balancing
3. Share Redis for session storage
4. Use external database for persistence

## Troubleshooting

### Service Won't Start

```bash
# Check logs
docker-compose -f config/docker/docker-compose.production.yml logs neurograph-api

# Check disk space
df -h

# Check memory
free -h

# Restart service
docker-compose -f config/docker/docker-compose.production.yml restart neurograph-api
```

### High Memory Usage

```bash
# Check container stats
docker stats

# Reduce memory limit
API_MEMORY_LIMIT=1G

# Restart with new limits
docker-compose -f config/docker/docker-compose.production.yml up -d
```

### Connection Issues

```bash
# Check firewall
sudo ufw status

# Check listening ports
sudo netstat -tulpn | grep :8000

# Test connectivity
curl -v http://localhost:8000/api/v1/health
```

### Performance Issues

```bash
# Check system resources
htop

# Check Docker resources
docker stats

# Review metrics in Grafana
# Navigate to System Overview dashboard
```

## Maintenance

### Regular Tasks

**Daily:**
- [ ] Monitor alerts
- [ ] Check error logs
- [ ] Verify backups

**Weekly:**
- [ ] Review metrics
- [ ] Check disk usage
- [ ] Update security patches

**Monthly:**
- [ ] Full backup test
- [ ] Performance review
- [ ] Capacity planning
- [ ] Update Docker images

## Support

- **Documentation:** https://github.com/dchrnv/neurograph/tree/main/docs
- **Issues:** https://github.com/dchrnv/neurograph/issues
- **Security:** security@neurograph.org

---

**Next Steps:**
- [Security Hardening Guide](./SECURITY.md)
- [Performance Tuning](./PERFORMANCE.md)
- [Kubernetes Deployment](./KUBERNETES.md)
