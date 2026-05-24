import { useEffect, useState } from 'react';
import { connectWS } from './ws/client';
import { useEngineStore } from './store/engine';
import { AdvisoryQueue } from './components/AdvisoryQueue';
import { Conversation } from './components/Conversation';
import { PhaseC } from './components/PhaseC';
import { Patterns } from './components/Patterns';
import './App.css';

type Tab = 'overview' | 'conversation' | 'phase-c' | 'patterns';

const TABS: { id: Tab; label: string }[] = [
  { id: 'overview',     label: 'Overview' },
  { id: 'conversation', label: 'Conversation' },
  { id: 'phase-c',      label: 'Phase C' },
  { id: 'patterns',     label: 'Patterns' },
];

export default function App() {
  const { snapshot, connected } = useEngineStore();
  const [tab, setTab] = useState<Tab>('overview');

  useEffect(() => { connectWS(); }, []);

  const fatiguePct = snapshot ? Math.round(snapshot.fatigue.current * 100) : 0;
  const pendingCount = snapshot?.phase_c?.pending_advisories.length ?? 0;

  return (
    <div className="app">
      <header className="header">
        <span className="logo">AXIOM</span>
        <nav className="tabs">
          {TABS.map((t) => (
            <button
              key={t.id}
              className={`tab-btn ${tab === t.id ? 'tab-active' : ''}`}
              onClick={() => setTab(t.id)}
            >
              {t.label}
              {t.id === 'phase-c' && pendingCount > 0 && (
                <span className="tab-badge">{pendingCount}</span>
              )}
            </button>
          ))}
        </nav>
        <div className="header-right">
          {snapshot && (
            <span className="engine-state" data-state={snapshot.engine_state}>
              {snapshot.engine_state}
            </span>
          )}
          <span className={`conn-badge ${connected ? 'conn-ok' : 'conn-off'}`}>
            {connected ? '● live' : '○ reconnecting'}
          </span>
        </div>
      </header>

      {!snapshot ? (
        <div className="waiting">Waiting for engine snapshot…</div>
      ) : (
        <main className="main">
          {tab === 'overview' && (
            <>
              <section className="card metrics-row">
                <Metric label="Tick"       value={snapshot.current_tick.toLocaleString()} />
                <Metric label="Event"      value={snapshot.current_event.toLocaleString()} />
                <Metric label="Hot path"   value={`${(snapshot.hot_path_ns / 1_000).toFixed(1)} µs`} />
                <Metric label="Fatigue"    value={`${fatiguePct}%`} />
                <Metric label="Tokens"     value={snapshot.over_domain.total_tokens.toLocaleString()} />
                <Metric label="Connections" value={snapshot.over_domain.total_connections.toLocaleString()} />
                <Metric label="Dreams"     value={snapshot.dream_phase_stats.cycles_completed.toString()} />
                <Metric label="Vetoes"     value={snapshot.guardian_stats.total_vetoes.toString()} />
              </section>

              {snapshot.phase_c && pendingCount > 0 && (
                <AdvisoryQueue
                  advisories={snapshot.phase_c.pending_advisories}
                  currentEvent={snapshot.current_event}
                />
              )}

              <section className="card">
                <h2>Fatigue</h2>
                <FatigueBar current={snapshot.fatigue.current} threshold={snapshot.fatigue.threshold} />
              </section>

              {snapshot.frame_weaver_stats && (
                <section className="card">
                  <h2>FrameWeaver</h2>
                  <div className="metrics-row">
                    <Metric label="Total frames"     value={snapshot.frame_weaver_stats.total_frames.toString()} />
                    <Metric label="In sutra"         value={snapshot.frame_weaver_stats.frames_in_sutra.toString()} />
                    <Metric label="Promotions/wake"  value={snapshot.frame_weaver_stats.promotions_since_wake.toString()} />
                  </div>
                </section>
              )}
            </>
          )}

          {tab === 'conversation' && <Conversation />}
          {tab === 'phase-c'      && <PhaseC />}
          {tab === 'patterns'     && <Patterns />}
        </main>
      )}
    </div>
  );
}

function Metric({ label, value }: { label: string; value: string }) {
  return (
    <div className="metric">
      <span className="metric-label">{label}</span>
      <span className="metric-value">{value}</span>
    </div>
  );
}

function FatigueBar({ current, threshold }: { current: number; threshold: number }) {
  const pct = Math.round(current * 100);
  const thresholdPct = Math.round(threshold * 100);
  const color = current >= threshold ? 'var(--red)' : current > threshold * 0.7 ? 'var(--yellow)' : 'var(--green)';
  return (
    <div className="fatigue-bar-wrap">
      <div className="fatigue-bar-track">
        <div className="fatigue-bar-fill" style={{ width: `${pct}%`, background: color }} />
        <div className="fatigue-bar-threshold" style={{ left: `${thresholdPct}%` }} title={`Threshold: ${thresholdPct}%`} />
      </div>
      <span className="fatigue-label">{pct}% / threshold {thresholdPct}%</span>
    </div>
  );
}
