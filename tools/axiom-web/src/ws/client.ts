import { EngineMessage, EngineEvent } from './protocol';
import { useEngineStore } from '../store/engine';

let ws: WebSocket | null = null;
let reconnectTimer: ReturnType<typeof setTimeout> | null = null;

export function connectWS(): void {
  if (ws?.readyState === WebSocket.OPEN || ws?.readyState === WebSocket.CONNECTING) {
    return;
  }

  const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  const url = `${proto}//${window.location.host}/api/ws`;

  ws = new WebSocket(url);

  ws.onopen = () => {
    useEngineStore.getState().setConnected(true);
    if (reconnectTimer !== null) {
      clearTimeout(reconnectTimer);
      reconnectTimer = null;
    }
  };

  ws.onmessage = (event: MessageEvent<string>) => {
    try {
      const msg = JSON.parse(event.data) as EngineMessage;
      handleMessage(msg);
    } catch {
      // ignore malformed frames
    }
  };

  ws.onclose = () => {
    useEngineStore.getState().setConnected(false);
    ws = null;
    reconnectTimer = setTimeout(connectWS, 2000);
  };

  ws.onerror = () => {
    ws?.close();
  };
}

function handleMessage(msg: EngineMessage): void {
  const store = useEngineStore.getState();

  if ('Snapshot' in msg) {
    store.setSnapshot(msg.Snapshot);
    return;
  }

  if ('Sensorium' in msg) {
    store.setSensorium(msg.Sensorium);
    return;
  }

  if ('Event' in msg) {
    handleEvent(msg.Event);
  }
}

function handleEvent(ev: EngineEvent): void {
  const store = useEngineStore.getState();
  const tick = store.snapshot?.current_tick;

  if ('FrameCrystallized' in ev) {
    const { anchor_id, participant_count } = ev.FrameCrystallized;
    store.addFeedMessage({
      kind: 'frame',
      text: `Frame #${anchor_id} crystallized  (${participant_count} participants)`,
      tick,
    });
    return;
  }

  if ('DreamPhaseTransition' in ev) {
    const { from, to } = ev.DreamPhaseTransition;
    store.addFeedMessage({
      kind: 'dream',
      text: `Engine: ${from} → ${to}`,
      tick,
    });
    return;
  }

  if ('Alert' in ev) {
    store.addFeedMessage({
      kind: 'alert',
      text: ev.Alert.message,
      tick,
    });
  }
}
