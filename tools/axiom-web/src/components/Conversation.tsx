import { useRef, useState, useEffect, KeyboardEvent } from 'react';
import { useEngineStore } from '../store/engine';

export function Conversation() {
  const { feed, snapshot } = useEngineStore();
  const [text, setText] = useState('');
  const [sending, setSending] = useState(false);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [feed.length]);

  async function submit() {
    const trimmed = text.trim();
    if (!trimmed || sending) return;
    setSending(true);
    useEngineStore.getState().addFeedMessage({
      kind: 'user',
      text: trimmed,
      tick: snapshot?.current_tick,
    });
    setText('');
    await fetch('/api/text/submit', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text: trimmed }),
    });
    setSending(false);
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
            <span className="feed-meta">
              {m.tick != null ? `t${m.tick.toLocaleString()}` : ''}
            </span>
            <span className="feed-text">{m.text}</span>
          </div>
        ))}
        <div ref={bottomRef} />
      </div>

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
    </div>
  );
}
