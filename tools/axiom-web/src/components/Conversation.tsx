import { useRef, useState, useEffect, KeyboardEvent } from 'react';
import { useEngineStore } from '../store/engine';

export function Conversation() {
  const { feed, snapshot } = useEngineStore();
  const [text, setText] = useState('');
  const [sending, setSending] = useState(false);
  const [batchMode, setBatchMode] = useState(false);
  const [batchText, setBatchText] = useState('');
  const [batchDelay, setBatchDelay] = useState(500);
  const [batchRunning, setBatchRunning] = useState(false);
  const [batchProgress, setBatchProgress] = useState<{ done: number; total: number } | null>(null);
  const batchStopRef = useRef(false);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [feed.length]);

  async function submit() {
    const trimmed = text.trim();
    if (!trimmed || sending) return;
    setSending(true);
    useEngineStore.getState().addFeedMessage({ kind: 'user', text: trimmed, tick: snapshot?.current_tick });
    setText('');
    await fetch('/api/text/submit', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text: trimmed }),
    });
    setSending(false);
  }

  async function startBatch() {
    const lines = batchText.split('\n').map((l) => l.trim()).filter(Boolean);
    if (lines.length === 0) return;
    batchStopRef.current = false;
    setBatchRunning(true);
    setBatchProgress({ done: 0, total: lines.length });

    for (let i = 0; i < lines.length; i++) {
      if (batchStopRef.current) break;
      const line = lines[i];
      useEngineStore.getState().addFeedMessage({ kind: 'user', text: line, tick: snapshot?.current_tick });
      await fetch('/api/text/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ text: line }),
      });
      setBatchProgress({ done: i + 1, total: lines.length });
      if (i < lines.length - 1 && !batchStopRef.current) {
        await new Promise((res) => setTimeout(res, batchDelay));
      }
    }

    setBatchRunning(false);
    setBatchProgress(null);
    if (!batchStopRef.current) setBatchText('');
  }

  function stopBatch() {
    batchStopRef.current = true;
  }

  function onKeyDown(e: KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      submit();
    }
  }

  return (
    <div className="conversation">
      <div className="feed">
        {feed.length === 0 && (
          <div className="feed-empty">No messages yet. Send text to inject it into the engine.</div>
        )}
        {feed.map((m) => (
          <div key={m.id} className={`feed-item feed-${m.kind}`}>
            <span className="feed-meta">{m.tick != null ? `t${m.tick.toLocaleString()}` : ''}</span>
            <span className="feed-text">{m.text}</span>
          </div>
        ))}
        <div ref={bottomRef} />
      </div>

      <div className="compose-mode-bar">
        <button
          className={`compose-mode-btn ${!batchMode ? 'compose-mode-active' : ''}`}
          onClick={() => setBatchMode(false)}
        >
          Single
        </button>
        <button
          className={`compose-mode-btn ${batchMode ? 'compose-mode-active' : ''}`}
          onClick={() => setBatchMode(true)}
        >
          Batch
        </button>
      </div>

      {!batchMode ? (
        <div className="compose">
          <textarea
            className="compose-input"
            value={text}
            onChange={(e) => setText(e.target.value)}
            onKeyDown={onKeyDown}
            placeholder="Type text to inject into AXIOM… (Ctrl+Enter to send)"
            rows={3}
          />
          <button className="compose-send" onClick={submit} disabled={sending || !text.trim()}>
            {sending ? '…' : 'Send'}
          </button>
        </div>
      ) : (
        <div className="batch-compose">
          <textarea
            className="compose-input batch-input"
            value={batchText}
            onChange={(e) => setBatchText(e.target.value)}
            placeholder={'Paste lines to inject — one per line.\nEach line is sent as a separate text injection.'}
            rows={8}
            disabled={batchRunning}
          />
          <div className="batch-controls">
            <label className="batch-delay-label">
              Delay
              <input
                className="batch-delay-input"
                type="number"
                min={50}
                max={10000}
                step={50}
                value={batchDelay}
                onChange={(e) => setBatchDelay(Math.max(50, Number(e.target.value)))}
                disabled={batchRunning}
              />
              ms
            </label>
            {batchProgress && (
              <span className="batch-progress">
                {batchProgress.done} / {batchProgress.total}
              </span>
            )}
            {!batchRunning ? (
              <button
                className="compose-send"
                onClick={startBatch}
                disabled={!batchText.trim()}
              >
                Start
              </button>
            ) : (
              <button className="compose-send batch-stop" onClick={stopBatch}>
                Stop
              </button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
