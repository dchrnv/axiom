import { useRef, useState, useEffect, KeyboardEvent } from 'react';
import { useEngineStore } from '../store/engine';

type GenMode = 'noise' | 'syllables' | 'words' | 'prose';

export function Conversation() {
  const { feed, snapshot } = useEngineStore();
  const [text, setText] = useState('');
  const [sending, setSending] = useState(false);

  // Batch
  const [batchMode, setBatchMode] = useState(false);
  const [batchText, setBatchText] = useState('');
  const [batchDelay, setBatchDelay] = useState(500);
  const [batchRunning, setBatchRunning] = useState(false);
  const [batchProgress, setBatchProgress] = useState<{ done: number; total: number } | null>(null);
  const batchStopRef = useRef(false);

  // Generator panel
  const [genOpen, setGenOpen] = useState(false);
  const [genMode, setGenMode] = useState<GenMode>('prose');
  const [genCount, setGenCount] = useState(20);
  const [genSeed, setGenSeed] = useState('');
  const [genLoading, setGenLoading] = useState(false);

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

  function stopBatch() { batchStopRef.current = true; }

  async function generateCorpus() {
    setGenLoading(true);
    const params = new URLSearchParams({ mode: genMode, count: String(genCount) });
    if (genSeed.trim()) params.set('seed', genSeed.trim());
    try {
      const res = await fetch(`/api/corpus/generate?${params}`);
      const data: { lines: string[] } = await res.json();
      setBatchText(data.lines.join('\n'));
      setBatchMode(true);
      setGenOpen(false);
    } finally {
      setGenLoading(false);
    }
  }

  function onKeyDown(e: KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) { e.preventDefault(); submit(); }
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
        <button className={`compose-mode-btn ${!batchMode ? 'compose-mode-active' : ''}`} onClick={() => setBatchMode(false)}>
          Single
        </button>
        <button className={`compose-mode-btn ${batchMode ? 'compose-mode-active' : ''}`} onClick={() => setBatchMode(true)}>
          Batch
        </button>
        <button className={`compose-mode-btn gen-btn ${genOpen ? 'compose-mode-active' : ''}`} onClick={() => setGenOpen((v) => !v)}>
          ⚡ Generate
        </button>
      </div>

      {genOpen && (
        <div className="gen-panel">
          <div className="gen-row">
            <span className="gen-label">Mode</span>
            <div className="gen-mode-group">
              {(['noise', 'syllables', 'words', 'prose'] as GenMode[]).map((m) => (
                <button
                  key={m}
                  className={`gen-mode-option ${genMode === m ? 'gen-mode-selected' : ''}`}
                  onClick={() => setGenMode(m)}
                >
                  {m}
                </button>
              ))}
            </div>
          </div>
          <div className="gen-row">
            <label className="gen-label" htmlFor="gen-count">Lines</label>
            <input
              id="gen-count"
              className="batch-delay-input"
              type="number"
              min={1}
              max={500}
              value={genCount}
              onChange={(e) => setGenCount(Math.max(1, Math.min(500, Number(e.target.value))))}
            />
            <label className="gen-label" htmlFor="gen-seed" style={{ marginLeft: 12 }}>Seed</label>
            <input
              id="gen-seed"
              className="batch-delay-input gen-seed-input"
              type="number"
              min={0}
              placeholder="random"
              value={genSeed}
              onChange={(e) => setGenSeed(e.target.value)}
            />
            <button className="compose-send gen-go-btn" onClick={generateCorpus} disabled={genLoading}>
              {genLoading ? '…' : 'Fill Batch'}
            </button>
          </div>
        </div>
      )}

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
              <span className="batch-progress">{batchProgress.done} / {batchProgress.total}</span>
            )}
            {!batchRunning ? (
              <button className="compose-send" onClick={startBatch} disabled={!batchText.trim()}>
                Start
              </button>
            ) : (
              <button className="compose-send batch-stop" onClick={stopBatch}>Stop</button>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
