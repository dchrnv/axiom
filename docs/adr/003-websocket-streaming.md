# ADR-003: WebSocket Event Streaming Architecture

**Status:** Accepted

**Date:** 2024-12-01

**Deciders:** Chernov Denys

## Context

Axiom needs real-time notification of state changes (token creation/updates, grid changes, CDNA updates). Clients need to react to these changes with minimal latency.

Requirements:
- Low latency (< 10ms preferred)
- Reliable delivery
- Support multiple concurrent clients
- Selective subscriptions (channels)
- Minimal server overhead

We needed to choose between polling, Server-Sent Events (SSE), and WebSocket.

## Decision

We will use **WebSocket for bidirectional real-time event streaming** with a **channel-based subscription model**.

**Architecture:**
- Single WebSocket endpoint: `ws://host/ws`
- Channel-based subscriptions: `tokens`, `grid`, `cdna`
- JSON message protocol
- Client-side subscription management
- Server-side event buffering (max 100 events per client)

**Message Types:**
- `subscribe` - Join a channel
- `unsubscribe` - Leave a channel
- `ping/pong` - Heartbeat
- Event messages - Broadcast to channel subscribers

## Consequences

### Positive

✅ **Low Latency:** Typical ~5ms event delivery (LAN)

✅ **Bidirectional:** Server can push, client can send commands

✅ **Efficient:** Single persistent connection vs HTTP polling

✅ **Selective Subscriptions:** Clients only receive relevant events

✅ **Standard Protocol:** Widely supported (browsers, Python, etc.)

✅ **Backpressure Handling:** Buffer prevents message loss during slowdowns

### Negative

❌ **Connection Management:** Need to handle reconnection logic

❌ **Stateful:** Server must track active connections

❌ **Firewall Issues:** Some corporate firewalls block WebSocket

❌ **Scaling Complexity:** Need shared pub/sub for multi-instance (future)

## Alternatives Considered

### 1. HTTP Polling

**Pros:**
- Simple implementation
- No connection state
- Firewall-friendly

**Cons:**
- High latency (1-5 seconds typical)
- Bandwidth waste (empty polls)
- Server load (constant requests)

**Rejected because:** Latency unacceptable for real-time updates

### 2. Server-Sent Events (SSE)

**Pros:**
- Simpler than WebSocket
- Built-in reconnection
- HTTP-based (proxy-friendly)

**Cons:**
- Unidirectional (server → client only)
- Limited browser connection pool (6 per domain)
- No binary support
- Less efficient than WebSocket

**Rejected because:** Need bidirectional communication for heartbeat

### 3. gRPC Streaming

**Pros:**
- Efficient binary protocol
- Built-in flow control
- Strong typing

**Cons:**
- Requires HTTP/2
- Limited browser support (needs grpc-web proxy)
- More complex setup
- Overkill for JSON events

**Rejected because:** WebSocket simpler and browser-native

### 4. MQTT

**Pros:**
- Designed for pub/sub
- QoS levels
- Efficient for IoT

**Cons:**
- Separate broker required
- Not browser-native
- Overkill for our use case

**Rejected because:** WebSocket sufficient

## Implementation

### Connection Manager

```python
class ConnectionManager:
    def __init__(self):
        self.active_connections: Dict[str, WebSocket] = {}
        self.subscriptions: Dict[str, Set[str]] = {}  # channel -> client_ids
        self.event_buffers: Dict[str, Deque] = {}  # client_id -> events
```

### Channels

- **`tokens`** - Token CRUD events (created, updated, deleted)
- **`grid`** - Grid structure changes
- **`cdna`** - CDNA configuration updates

### Message Protocol

```json
// Subscribe
{
  "type": "subscribe",
  "channel": "tokens"
}

// Event notification
{
  "type": "token_created",
  "channel": "tokens",
  "data": {
    "token_id": 123,
    "position": [1.0, 2.0, ...],
    "timestamp": "2026-01-09T10:30:00Z"
  }
}

// Heartbeat
{
  "type": "ping"
}
// Response
{
  "type": "pong",
  "timestamp": "2026-01-09T10:30:00Z"
}
```

### Event Buffer

**Purpose:** Prevent message loss during temporary client slowdowns

**Configuration:**
- Max buffer size: 100 events per client
- Overflow behavior: Drop oldest events
- Metric: `ws_buffer_overflow_total`

### Error Handling

**Client disconnection:**
- Clean up subscriptions
- Clear event buffer
- Update metrics

**Connection errors:**
- Client should implement exponential backoff
- Reconnect attempts: 3 with 2^n second delays

## Performance Characteristics

**Latency:** ~5ms (LAN), ~50ms (WAN)

**Throughput:** ~10,000 events/sec per instance

**Memory:** ~10KB per connection + buffer

**Scalability:**
- Single instance: 1000-10,000 concurrent connections
- Multi-instance: Requires Redis pub/sub (future)

## Monitoring

**Prometheus Metrics:**
- `axiom_ws_connections_total` - Active connections
- `axiom_ws_messages_sent_total` - Events broadcast
- `axiom_ws_message_latency_seconds` - Delivery latency
- `axiom_ws_buffer_overflow_total` - Dropped events
- `axiom_ws_disconnects_total` - Disconnection reasons

## Future Enhancements

### v1.1: Message Compression
- Gzip compression for large events
- Reduce bandwidth 60-80%

### v1.2: Multi-Instance Support
- Redis pub/sub for event distribution
- Shared connection state
- Load balancer with sticky sessions

### v2.0: Event Replay
- Persist events to database
- Allow clients to request missed events
- Useful for reconnection scenarios

### v2.0: Filtered Subscriptions
- Subscribe with filters: `{"channel": "tokens", "filter": {"user_id": 123}}`
- Reduce irrelevant events

## Security Considerations

**Authentication:**
- JWT token in query parameter or first message
- Validate before accepting subscriptions

**Rate Limiting:**
- Max subscriptions per client: 10
- Max messages per second: 100
- Disconnect abusive clients

**Data Exposure:**
- Only send events for resources client has access to
- Respect permission scopes

## References

- [WebSocket RFC 6455](https://tools.ietf.org/html/rfc6455)
- [FastAPI WebSocket Documentation](https://fastapi.tiangolo.com/advanced/websockets/)
- Axiom WebSocket Metrics (production)

## Revision History

- 2024-12-01: Initial decision
- 2024-12-15: Added buffer and metrics
- 2026-01-09: Production experience and scaling notes
