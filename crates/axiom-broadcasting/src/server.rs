use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tracing::{debug, info, warn};

use axiom_protocol::commands::EngineCommand;
use axiom_protocol::messages::{ClientMessage, EngineMessage, ShutdownReason};
use axiom_protocol::snapshot::SystemSnapshot;
use axiom_protocol::PROTOCOL_VERSION;

use crate::config::BroadcastingConfig;

static CLIENT_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Handle held by the Engine's tick loop.
/// All methods are sync-safe — no async required on the Engine side.
pub struct BroadcastHandle {
    /// Engine publishes events/snapshots here.
    pub event_tx: broadcast::Sender<EngineMessage>,
    /// Engine polls pending commands from clients here.
    pub command_rx: Mutex<mpsc::UnboundedReceiver<(u64, EngineCommand)>>,
    /// Pre-serialized EngineMessage::Snapshot sent to each new client on connect.
    snapshot_cache: RwLock<Option<Vec<u8>>>,
}

impl BroadcastHandle {
    /// Non-blocking send of an EngineMessage to all subscribed clients.
    /// Silently drops if no subscribers.
    pub fn publish(&self, msg: EngineMessage) {
        let _ = self.event_tx.send(msg);
    }

    /// Non-blocking poll: returns one pending command if available.
    pub async fn try_recv_command(&self) -> Option<(u64, EngineCommand)> {
        self.command_rx.lock().await.try_recv().ok()
    }

    /// Update the cached snapshot sent to new clients on connect.
    /// Called by Engine after each tick or on demand.
    pub fn update_snapshot(&self, snap: SystemSnapshot) {
        if let Ok(bytes) = postcard::to_stdvec(&EngineMessage::Snapshot(snap)) {
            if let Ok(mut guard) = self.snapshot_cache.write() {
                *guard = Some(bytes);
            }
        }
    }
}

pub struct BroadcastServer {
    config: BroadcastingConfig,
    addr: SocketAddr,
    handle: Arc<BroadcastHandle>,
    command_tx: mpsc::UnboundedSender<(u64, EngineCommand)>,
}

impl BroadcastServer {
    /// Create server and return (server, handle).
    /// Call `server.run().await` in a tokio task to start accepting connections.
    pub fn new(addr: SocketAddr, config: BroadcastingConfig) -> (Self, Arc<BroadcastHandle>) {
        let (event_tx, _) = broadcast::channel(config.max_event_queue_per_client);
        let (command_tx, command_rx) = mpsc::unbounded_channel();

        let handle = Arc::new(BroadcastHandle {
            event_tx: event_tx.clone(),
            command_rx: Mutex::new(command_rx),
            snapshot_cache: RwLock::new(None),
        });

        let server = BroadcastServer {
            config,
            addr,
            handle: handle.clone(),
            command_tx,
        };

        (server, handle)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.addr).await?;
        info!("BroadcastServer listening on {}", self.addr);

        let server = Arc::new(self);

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            debug!("New connection from {}", peer_addr);
            let server_clone = server.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.handle_connection(stream, peer_addr).await {
                    warn!("Connection {} error: {}", peer_addr, e);
                }
            });
        }
    }

    async fn handle_connection(
        &self,
        stream: TcpStream,
        peer_addr: SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sink, mut ws_source) = ws_stream.split();

        // Handshake: wait for ClientMessage::Hello
        let hello_bytes = match ws_source.next().await {
            Some(Ok(WsMessage::Binary(b))) => b,
            _ => {
                warn!("No Hello from {}", peer_addr);
                return Ok(());
            }
        };

        let client_hello: ClientMessage = match postcard::from_bytes(&hello_bytes) {
            Ok(m) => m,
            Err(_) => {
                warn!("Bad Hello from {}", peer_addr);
                return Ok(());
            }
        };

        let (client_version, client_kind, subscriptions) = match client_hello {
            ClientMessage::Hello {
                version,
                client_kind,
            } => (
                version,
                client_kind,
                axiom_protocol::event_category::DEFAULT,
            ),
            _ => {
                warn!("Expected Hello, got other message from {}", peer_addr);
                return Ok(());
            }
        };

        // Version check: major must match
        if (client_version >> 24) != (PROTOCOL_VERSION >> 24) {
            let bye = EngineMessage::Bye {
                reason: ShutdownReason::VersionMismatch,
            };
            let bytes = postcard::to_stdvec(&bye)?;
            ws_sink.send(WsMessage::Binary(bytes)).await?;
            return Ok(());
        }

        let client_id = CLIENT_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        info!(
            "Client {} connected: id={} kind={:?}",
            peer_addr, client_id, client_kind
        );

        // Send Hello back
        let server_hello = EngineMessage::Hello {
            version: PROTOCOL_VERSION,
            capabilities: 0,
        };
        let bytes = postcard::to_stdvec(&server_hello)?;
        ws_sink.send(WsMessage::Binary(bytes)).await?;

        // Send current snapshot so client has initial state without waiting for next event
        let cached_snapshot = self
            .handle
            .snapshot_cache
            .read()
            .ok()
            .and_then(|g| g.as_ref().cloned());
        if let Some(snap_bytes) = cached_snapshot {
            ws_sink.send(WsMessage::Binary(snap_bytes)).await?;
        }

        // Subscribe to broadcast channel
        let mut event_rx = self.handle.event_tx.subscribe();
        let command_tx = self.command_tx.clone();
        let mut subscribed_categories = subscriptions;
        let tick_event_interval = self.config.tick_event_interval;
        let domain_activity_threshold = self.config.domain_activity_threshold;

        // Heartbeat: server initiates ping every heartbeat_interval, expects pong within pong_timeout.
        let heartbeat_interval = self.config.heartbeat_interval;
        let pong_timeout_dur = self.config.pong_timeout;
        let mut heartbeat = tokio::time::interval(heartbeat_interval);
        heartbeat.tick().await; // skip the immediate first tick
        let mut pong_expires_at: Option<tokio::time::Instant> = None;

        // Main client loop
        let mut ws_sink = ws_sink;
        loop {
            let pong_deadline = pong_expires_at; // Copy into loop body for the async block

            tokio::select! {
                // Pong timeout: fires when we sent a ping and no pong arrived in time
                _ = async move {
                    match pong_deadline {
                        Some(d) => tokio::time::sleep_until(d).await,
                        None    => std::future::pending::<()>().await,
                    }
                } => {
                    warn!("Client {} pong timeout, disconnecting", client_id);
                    break;
                }

                // Heartbeat: send ping to client (skip if already waiting for pong)
                _ = heartbeat.tick(), if pong_expires_at.is_none() => {
                    if ws_sink.send(WsMessage::Ping(vec![])).await.is_err() {
                        break;
                    }
                    pong_expires_at = Some(tokio::time::Instant::now() + pong_timeout_dur);
                }

                // Outbound: Engine events → client
                result = event_rx.recv() => {
                    match result {
                        Ok(msg) => {
                            if !should_send(&msg, subscribed_categories, tick_event_interval, domain_activity_threshold) {
                                continue;
                            }
                            let bytes = match postcard::to_stdvec(&msg) {
                                Ok(b) => b,
                                Err(_) => continue,
                            };
                            if ws_sink.send(WsMessage::Binary(bytes)).await.is_err() {
                                break;
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            warn!("Client {} lagged, dropped {} messages — sending snapshot for resync", client_id, n);
                            let snap_bytes = self
                                .handle
                                .snapshot_cache
                                .read()
                                .ok()
                                .and_then(|g| g.as_ref().cloned());
                            if let Some(bytes) = snap_bytes {
                                if ws_sink.send(WsMessage::Binary(bytes)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }

                // Inbound: client commands → Engine
                result = ws_source.next() => {
                    match result {
                        Some(Ok(WsMessage::Binary(bytes))) => {
                            match postcard::from_bytes::<ClientMessage>(&bytes) {
                                Ok(ClientMessage::Command { command_id, command }) => {
                                    let _ = command_tx.send((command_id, command));
                                }
                                Ok(ClientMessage::Subscribe { event_categories }) => {
                                    subscribed_categories = event_categories;
                                }
                                Ok(ClientMessage::Bye) | Err(_) => break,
                                Ok(_) => {}
                            }
                        }
                        Some(Ok(WsMessage::Ping(data))) => {
                            let _ = ws_sink.send(WsMessage::Pong(data)).await;
                        }
                        Some(Ok(WsMessage::Pong(_))) => {
                            pong_expires_at = None;
                        }
                        Some(Ok(WsMessage::Close(_))) | None => break,
                        _ => {}
                    }
                }
            }
        }

        info!("Client {} disconnected", client_id);
        Ok(())
    }
}

fn should_send(
    msg: &EngineMessage,
    categories: u64,
    tick_event_interval: u32,
    domain_activity_threshold: u32,
) -> bool {
    use axiom_protocol::event_category::*;
    use axiom_protocol::events::EngineEvent;

    match msg {
        EngineMessage::Event(ev) => {
            let cat = match ev {
                EngineEvent::Tick { tick, .. } => {
                    if tick % tick_event_interval as u64 != 0 {
                        return false;
                    }
                    TICK
                }
                EngineEvent::DomainActivity { recent_activity, .. } => {
                    if *recent_activity < domain_activity_threshold {
                        return false;
                    }
                    DOMAIN_ACTIVITY
                }
                EngineEvent::DreamPhaseTransition { .. } => DREAM_PHASE,
                EngineEvent::FrameCrystallized { .. }
                | EngineEvent::FrameReactivated { .. }
                | EngineEvent::FramePromoted { .. } => FRAMES,
                EngineEvent::GuardianVeto { .. } => GUARDIAN,
                EngineEvent::AdapterStarted { .. }
                | EngineEvent::AdapterProgress { .. }
                | EngineEvent::AdapterFinished { .. } => ADAPTERS,
                EngineEvent::BenchStarted { .. }
                | EngineEvent::BenchProgress { .. }
                | EngineEvent::BenchFinished { .. } => BENCHMARKS,
                EngineEvent::Alert { .. } => ALERTS,
            };
            categories & cat != 0
        }
        // Snapshot, Hello, Bye, CommandResult always go through
        _ => true,
    }
}
