import { useEffect, useState } from 'react';
import { connectWS } from './ws/client';
import { useEngineStore } from './store/engine';
import { AdvisoryQueue } from './components/AdvisoryQueue';
import { Conversation } from './components/Conversation';
import { PhaseC } from './components/PhaseC';
import { Patterns } from './components/Patterns';
import { Domains } from './components/Domains';
import { Traces } from './components/Traces';
import { Internals } from './components/Internals';
import { Lab } from './components/Lab';
import { Charts } from './components/Charts';
import './App.css';

type Tab = 'overview' | 'domains' | 'traces' | 'internals' | 'conversation' | 'phase-c' | 'patterns' | 'charts' | 'lab';

const TABS: { id: Tab; label: string }[] = [
  { id: 'overview',     label: 'Overview' },
  { id: 'charts',       label: 'Charts' },
  { id: 'domains',      label: 'Domains' },
  { id: 'traces',       label: 'Traces' },
  { id: 'internals',    label: 'Internals' },
  { id: 'conversation', label: 'Conversation' },
  { id: 'phase-c',      label: 'Phase C' },
  { id: 'patterns',     label: 'Patterns' },
  { id: 'lab',          label: 'Lab' },
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

      {tab === 'lab' && <main className="main"><Lab /></main>}

      {tab !== 'lab' && !snapshot && (
        <div className="waiting">Waiting for engine snapshot…</div>
      )}

      {tab !== 'lab' && snapshot && (
        <main className="main">
          {tab === 'overview' && (
            <>
              <section className="card metrics-row">
                <Metric label="Tick"        value={snapshot.current_tick.toLocaleString()} />
                <Metric label="Event"       value={snapshot.current_event.toLocaleString()} />
                <Metric label="Hz"          value={snapshot.perf.actual_hz.toFixed(1)} />
                <Metric label="Avg tick"    value={fmtNs(snapshot.perf.tick_ns_avg)} />
                <Metric label="Peak tick"   value={fmtNs(snapshot.perf.tick_ns_peak)} />
                <Metric label="Uptime"      value={fmtUptime(snapshot.perf.uptime_secs)} />
                <Metric label="Fatigue"     value={`${fatiguePct}%`} />
                <Metric label="Tokens"      value={snapshot.over_domain.total_tokens.toLocaleString()} />
                <Metric label="Connections" value={snapshot.over_domain.total_connections.toLocaleString()} />
                <Metric label="Traces"      value={snapshot.traces_count.toLocaleString()} />
                <Metric label="Tension"     value={snapshot.tension_count.toString()} />
                <Metric label="Skills"      value={snapshot.skills_count.toString()} />
                <Metric label="Dreams"      value={snapshot.dream_phase_stats.cycles_completed.toString()} />
                <Metric label="Vetoes"      value={snapshot.guardian_stats.total_vetoes.toString()} />
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

          {tab === 'domains'      && <Domains />}
          {tab === 'traces'       && <Traces />}
          {tab === 'internals'    && <Internals />}
          {tab === 'conversation' && <Conversation />}
          {tab === 'phase-c'      && <PhaseC />}
          {tab === 'patterns'     && <Patterns />}
          {tab === 'charts'       && <Charts />}
        </main>
      )}
    </div>
  );
}

function fmtNs(ns: number): string {
  if (ns === 0) return '—';
  if (ns < 1_000) return `${ns} ns`;
  if (ns < 1_000_000) return `${(ns / 1_000).toFixed(1)} µs`;
  return `${(ns / 1_000_000).toFixed(2)} ms`;
}

function fmtUptime(secs: number): string {
  if (secs < 60) return `${secs.toFixed(0)}s`;
  if (secs < 3600) return `${Math.floor(secs / 60)}m ${Math.floor(secs % 60)}s`;
  return `${Math.floor(secs / 3600)}h ${Math.floor((secs % 3600) / 60)}m`;
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
