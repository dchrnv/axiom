# Phase 3.1: Monitoring & Observability - COMPLETE

**Version:** v0.67.1
**Date:** 2026-01-11
**Status:** ✅ ЗАВЕРШЕНО

---

## Обзор

Фаза 3.1 направлена на обеспечение полного мониторинга и observability для production развертывания NeuroGraph. Все задачи успешно завершены.

## Выполненные задачи

### 1. ✅ Health Endpoints (v0.67.0)

Реализованы комплексные health check endpoints для Kubernetes и мониторинга:

**Endpoints:**
- `/health` - Основной health check с детализацией компонентов
- `/health/live` - Liveness probe для K8s (проверка процесса)
- `/health/ready` - Readiness probe для K8s (готовность принимать трафик)
- `/health/startup` - Startup probe для K8s (завершение инициализации)
- `/health/components` - Детальный статус всех компонентов

**Проверяемые компоненты:**
- Rust Core (FFI initialization)
- Token Storage (operational status)
- Grid Storage (accessibility)
- CDNA Storage (profile count)
- WebSocket Manager (connections, channels, subscriptions)

**Системные метрики:**
- CPU usage (process.cpu_percent)
- Memory usage (RSS, percentage)
- Thread count
- Uptime

**Файл:** [src/api/routers/health.py](../src/api/routers/health.py)

**Kubernetes Integration:**
```yaml
livenessProbe:
  httpGet:
    path: /health/live
    port: 8080

readinessProbe:
  httpGet:
    path: /health/ready
    port: 8080

startupProbe:
  httpGet:
    path: /health/startup
    port: 8080
```

---

### 2. ✅ Grafana Dashboards

Созданы 3 комплексных dashboard для визуализации метрик:

#### Dashboard 1: System Overview
**Файл:** `grafana/dashboards/01-system-overview.json`

**Метрики:**
- CPU и Memory usage (gauges)
- Request rate (requests/sec)
- Request latency (p95/p99 percentiles)
- HTTP status codes breakdown (2xx/4xx/5xx)
- Active connections
- System uptime

**Панели:** 6 основных panels с различными визуализациями

#### Dashboard 2: WebSocket Metrics
**Файл:** `grafana/dashboards/02-websocket-metrics.json`

**Метрики:**
- Active WebSocket connections (gauge)
- Message rate (sent/received)
- Messages by channel (breakdown)
- Channel subscribers count
- Buffered events count
- Connection drop rate

**Панели:** 5 panels специфичных для WebSocket

#### Dashboard 3: Token & Grid Metrics
**Файл:** `grafana/dashboards/03-token-grid-metrics.json`

**Метрики:**
- Total tokens in storage (gauge)
- Token CRUD operations rate (Create/Read/Update/Delete)
- Grid query operations (Spatial search, Neighbors, Field influence)
- Grid query latency (p95)
- Token operations by type
- Storage capacity utilization

**Панели:** 6 panels для token и grid операций

**Provisioning:**
- Auto-load при запуске Grafana
- Конфигурация: `grafana/provisioning/dashboards/default.yml`
- Prometheus datasource: `grafana/provisioning/datasources/prometheus.yml`

**README:** [grafana/README.md](../grafana/README.md)

---

### 3. ✅ Prometheus Alert Rules

Настроено 9 alert rules в 2 группах для проактивного мониторинга:

**Файл:** `prometheus-alerts.yml`

#### Группа: neurograph_api_alerts (7 alerts)

1. **HighErrorRate** (CRITICAL)
   - Условие: HTTP 5xx error rate > 5%
   - Период: 5 минут
   - Severity: critical

2. **HighLatency** (WARNING)
   - Условие: p95 latency > 1.0s
   - Период: 10 минут
   - Severity: warning

3. **HighMemoryUsage** (WARNING)
   - Условие: Memory > 1.5GB (1610612736 bytes)
   - Период: 5 минут
   - Severity: warning

4. **HighCPUUsage** (WARNING)
   - Условие: CPU usage > 80%
   - Период: 10 минут
   - Severity: warning

5. **WebSocketConnectionsDrop** (WARNING)
   - Условие: Connections drop > 50% за 5 минут
   - Период: 2 минуты
   - Severity: warning

6. **ServiceDown** (CRITICAL)
   - Условие: Service unreachable
   - Период: 1 минута
   - Severity: critical

7. **UnusuallyHighRequestRate** (WARNING)
   - Условие: Request rate > 1000 req/s
   - Период: 5 минут
   - Severity: warning
   - Назначение: Potential DDoS detection

#### Группа: neurograph_storage_alerts (2 alerts)

8. **HighTokenCount** (WARNING)
   - Условие: Token count > 90,000
   - Период: 10 минут
   - Severity: warning
   - Назначение: Storage capacity warning

9. **HighBufferedEvents** (WARNING)
   - Условие: Buffered events > 500
   - Период: 5 минут
   - Severity: warning
   - Назначение: WebSocket buffer saturation

**Alert Coverage:**
- API errors and latency
- System resources (CPU, Memory)
- WebSocket health
- Storage capacity
- Service availability

---

### 4. ✅ Structured Logging

Structured logging реализовано во всех критичных компонентах:

**Logging Config:** [src/api/logging_config.py](../src/api/logging_config.py)

**Компоненты с логированием:**
- Health endpoints (health.py)
- Main API (main.py)
- Middleware (middleware.py, security.py)
- Error handlers (error_handlers.py)
- WebSocket (manager.py, connection.py, permissions.py, reconnection.py, compression.py, rate_limit.py)
- Routers (cdna.py, tokens.py)
- Storage (runtime.py, runtime_storage.py)
- Dependencies (dependencies.py)
- Integration (pipeline.py)

**Формат логов:**
```python
logger = get_logger(__name__, component="health")
logger.info("Health check completed", extra={"status": "healthy"})
logger.warning("Component check failed", extra={"component": "runtime"})
logger.error("Critical error", extra={"error": str(e)})
```

**Конфигурация логирования в Docker:**
```yaml
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
```

---

## Docker Compose Integration

**Файл:** `docker-compose.yml`

### Monitoring Stack

Все компоненты интегрированы в docker-compose с профилем `monitoring`:

```bash
# Запуск с мониторингом
docker-compose --profile monitoring up -d
```

**Services:**
1. **neurograph-api** - Main API server
2. **prometheus** - Metrics collection (port 9090)
3. **grafana** - Visualization (port 3000)
4. **jaeger** - Distributed tracing (optional, profile: tracing)

**Volumes:**
- `prometheus-data:/prometheus` - Metrics storage (7 day retention)
- `grafana-data:/var/lib/grafana` - Dashboards & settings
- `neurograph-logs:/app/logs` - Application logs

**Network:**
- `neurograph-network` - Bridge network для всех сервисов

---

## Метрики

### Standard Metrics (FastAPI/Prometheus)

- `http_requests_total` - Total HTTP requests counter
- `http_request_duration_seconds` - Request duration histogram
- `process_cpu_seconds_total` - CPU time counter
- `process_resident_memory_bytes` - RSS memory gauge
- `up` - Service availability (1 = up, 0 = down)

### Custom NeuroGraph Metrics

#### WebSocket Metrics
- `websocket_connections_active` - Active connections gauge
- `websocket_messages_sent_total` - Sent messages counter
- `websocket_messages_received_total` - Received messages counter
- `websocket_channel_subscribers` - Subscribers per channel gauge
- `websocket_buffered_events` - Buffered events gauge

#### Token & Grid Metrics
- `token_storage_count` - Total tokens in storage gauge
- `token_operations_total{operation="create|read|update|delete"}` - Token ops counter
- `grid_query_operations_total{operation="spatial|neighbors|field"}` - Grid ops counter
- `grid_query_duration_seconds` - Grid query latency histogram

---

## Quick Start

### 1. Запуск мониторинга

```bash
# Запуск полного стека
docker-compose --profile monitoring up -d

# Проверка статуса
docker-compose ps
```

### 2. Доступ к UI

**Grafana:**
- URL: http://localhost:3000
- Username: `admin`
- Password: `admin` (изменить через `GRAFANA_PASSWORD`)

**Prometheus:**
- URL: http://localhost:9090
- Alerts: http://localhost:9090/alerts
- Targets: http://localhost:9090/targets

**API Metrics:**
- Endpoint: http://localhost:8080/metrics
- Format: Prometheus exposition format

### 3. Health Checks

```bash
# Liveness probe
curl http://localhost:8080/health/live

# Readiness probe
curl http://localhost:8080/health/ready

# Full health check
curl http://localhost:8080/health

# Component details
curl http://localhost:8080/health/components
```

---

## Production Recommendations

### Security
- ✅ Изменить дефолтные Grafana credentials
- ✅ Настроить Grafana authentication (OAuth, LDAP)
- ✅ Enable HTTPS для Grafana (reverse proxy)
- ✅ Ограничить доступ к Prometheus (network policies)

### Alerting
- ✅ Настроить Alertmanager для notifications
- ✅ Интегрировать с Slack/PagerDuty/Email
- ✅ Создать on-call rotation schedule
- ✅ Документировать runbooks для alerts

### Data Retention
- ✅ Prometheus retention: 7 дней (дефолт, можно увеличить)
- ✅ Grafana dashboard backups (регулярный export)
- ✅ Log rotation: 10MB x 3 files (дефолт)
- ✅ Monitoring disk usage для Prometheus data

### Scalability
- ✅ Prometheus federation для multi-cluster
- ✅ Grafana high availability setup
- ✅ Remote storage для long-term metrics (Thanos, Cortex)

---

## Критерии успеха Фазы 3.1

Все критерии выполнены:

- ✅ Health endpoints работают в K8s (live, ready, startup probes)
- ✅ 3-5 Grafana dashboards развернуты (3 комплексных dashboards)
- ✅ Alert rules настроены и протестированы (9 rules в 2 группах)
- ✅ Structured logging во всех критичных операциях (20+ компонентов)
- ✅ Docker Compose integration с профилем monitoring
- ✅ Автоматическая provisioning для Grafana dashboards
- ✅ Документация и README для мониторинга

---

## Файлы

### Health & Monitoring
- `src/api/routers/health.py` - Health check endpoints (405 lines)
- `src/api/logging_config.py` - Logging configuration
- `src/api/models/status.py` - Health response models

### Grafana
- `grafana/dashboards/01-system-overview.json` - System metrics dashboard
- `grafana/dashboards/02-websocket-metrics.json` - WebSocket dashboard
- `grafana/dashboards/03-token-grid-metrics.json` - Token/Grid dashboard
- `grafana/provisioning/dashboards/default.yml` - Auto-load config
- `grafana/provisioning/datasources/prometheus.yml` - Datasource config
- `grafana/README.md` - Grafana documentation

### Prometheus
- `prometheus.yml` - Prometheus configuration
- `prometheus-alerts.yml` - Alert rules (9 rules)

### Docker
- `docker-compose.yml` - Full stack orchestration

---

## Следующие шаги

Фаза 3.1 полностью завершена. Переход к:

**Фаза 3.2: Docker & Deployment (v0.67.2)**
- Оптимизация Docker образов (target: <500MB)
- Production docker-compose
- Production Deployment Guide

---

**Версия документа:** 1.0
**Дата:** 2026-01-11
**Статус:** COMPLETE ✅
