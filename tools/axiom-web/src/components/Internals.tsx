import { useEngineStore } from '../store/engine';
import type { NeuralDepthStatus, SubsystemActivity } from '../ws/protocol';

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

export function Internals() {
  const { snapshot, sensorium } = useEngineStore();
  if (!snapshot) return <div className="waiting">Waiting for snapshot…</div>;

  const { perf, reflector, cognitive_depth } = snapshot;

  const maxReflectorRate = Math.max(...reflector.per_domain.map((d) => d.success_rate), 0.001);

  return (
    <div className="internals-view">
      {/* Subsystem Health */}
      {sensorium && sensorium.active_subsystems.length > 0 && (
        <section className="card">
          <h2>Subsystem Health</h2>
          <div className="sen-subsys">
            <div className="sen-subsys-header">
              <span>Subsystem</span>
              <span>Energy</span>
              <span>Fatigue</span>
            </div>
            {sensorium.active_subsystems.map((ss) => (
              <SubsysHealthRow key={ss.id} ss={ss} fatigued={sensorium.fatigued_subsystems.includes(ss.id)} />
            ))}
          </div>
          {sensorium.fatigued_subsystems.length > 0 && (
            <div className="sen-fatigued">
              <span className="sen-fatigued-label">Fatigued:</span>
              {sensorium.fatigued_subsystems.map((id) => (
                <span key={id} className="sen-fatigued-tag">{id}</span>
              ))}
            </div>
          )}
        </section>
      )}

      {/* Neural Depth Advisor */}
      {sensorium?.neural_depth && (
        <NeuralDepthPanel status={sensorium.neural_depth} />
      )}

      {/* Performance */}
      <section className="card">
        <h2>Performance</h2>
        <div className="internals-grid">
          <InternalRow label="Uptime"       value={fmtUptime(perf.uptime_secs)} />
          <InternalRow label="Actual Hz"    value={`${perf.actual_hz.toFixed(1)} Hz`} />
          <InternalRow label="Total ticks"  value={perf.total_ticks.toLocaleString()} />
          <InternalRow label="Avg tick"     value={fmtNs(perf.tick_ns_avg)} />
          <InternalRow label="Peak tick"    value={fmtNs(perf.tick_ns_peak)} />
        </div>
      </section>

      {/* Cognitive Depth */}
      <section className="card">
        <h2>Cognitive Depth (MAYA)</h2>
        <div className="internals-grid">
          <InternalRow label="Max passes"          value={cognitive_depth.max_passes.toString()} />
          <InternalRow label="Min coherence"       value={cognitive_depth.min_coherence.toFixed(3)} />
          <InternalRow label="Internal dominance"  value={cognitive_depth.internal_dominance.toFixed(3)} />
        </div>
      </section>

      {/* Reflector */}
      <section className="card">
        <h2>Reflector</h2>
        <div className="internals-grid" style={{ marginBottom: 14 }}>
          <InternalRow label="Patterns tracked" value={reflector.patterns_tracked.toLocaleString()} />
          <InternalRow label="Total success"    value={reflector.total_success.toLocaleString()} />
          <InternalRow label="Total fail"       value={reflector.total_fail.toLocaleString()} />
        </div>
        {reflector.per_domain.length === 0 ? (
          <div className="waiting" style={{ height: 50 }}>No reflector data yet</div>
        ) : (
          <table className="reflector-table">
            <thead>
              <tr>
                <th>Role</th>
                <th>Domain</th>
                <th>Success</th>
                <th>Total</th>
                <th>Rate</th>
                <th>Bar</th>
              </tr>
            </thead>
            <tbody>
              {reflector.per_domain.map((d) => (
                <tr key={d.role}>
                  <td className="refl-role">{d.role}</td>
                  <td className="refl-name">{d.name}</td>
                  <td className="refl-success">{d.success}</td>
                  <td className="refl-total">{d.total}</td>
                  <td className="refl-rate">{(d.success_rate * 100).toFixed(1)}%</td>
                  <td>
                    <div className="refl-bar-track">
                      <div
                        className="refl-bar-fill"
                        style={{
                          width: `${(d.success_rate / maxReflectorRate) * 100}%`,
                          background: d.success_rate > 0.7 ? 'var(--green)'
                            : d.success_rate > 0.4 ? 'var(--yellow)'
                            : 'var(--red)',
                        }}
                      />
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </section>
    </div>
  );
}

function InternalRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="internal-row">
      <span className="internal-label">{label}</span>
      <span className="internal-value">{value}</span>
    </div>
  );
}

const OCTANT_NAMES = ['O1 Apollo/Eros/Will', 'O2 Dio/Eros/Will', 'O3 Dio/Than/Will', 'O4 Apollo/Than/Will',
                      'O5 Apollo/Eros/Nth', 'O6 Dio/Eros/Nth', 'O7 Dio/Than/Nth', 'O8 Apollo/Than/Nth'];

function NeuralDepthPanel({ status }: { status: NeuralDepthStatus }) {
  const modeColor = status.mode === 'neural' ? 'var(--accent)' : 'var(--text-muted)';
  const maxW = Math.max(...status.cached_weights, 0.001);

  return (
    <section className="card">
      <h2>Neural Depth Advisor</h2>
      <div className="internals-grid" style={{ marginBottom: 12 }}>
        <InternalRow label="Mode" value={status.mode.toUpperCase()} />
        <InternalRow label="Last inference" value={fmtNs(status.last_infer_ns)} />
        <InternalRow label="Last infer tick" value={status.last_infer_tick.toLocaleString()} />
        <InternalRow label="Weights loaded" value={status.weights_loaded ? '✓ yes' : '— zeros'} />
      </div>
      <div style={{ fontSize: 11, color: 'var(--text-muted)', marginBottom: 6 }}>
        Reactivation weights (higher = needs reactivation):
      </div>
      {status.cached_weights.slice(0, 8).map((w, i) => (
        <div key={i} className="neural-weight-row">
          <span className="neural-weight-label">{OCTANT_NAMES[i] || `O${i+1}`}</span>
          <div className="sen-bar-track" style={{ flex: 1 }}>
            <div className="sen-bar-fill"
              style={{ width: `${(w / maxW) * 100}%`, background: w > 0.5 ? 'var(--accent)' : 'var(--green)' }} />
          </div>
          <span className="neural-weight-val">{(w * 100).toFixed(0)}%</span>
        </div>
      ))}
      <div style={{ fontSize: 11, color: modeColor, marginTop: 8 }}>
        {status.mode === 'neural'
          ? '● Neural model active — inference every 11 ticks'
          : '○ Rule-based mode — set mode: neural in genome.yaml to activate'}
      </div>
    </section>
  );
}

function SubsysHealthRow({ ss, fatigued }: { ss: SubsystemActivity; fatigued: boolean }) {
  const energyPct = Math.min((ss.energy / 255) * 100, 100);
  const fatiguePct = Math.min(ss.fatigue_load * 100, 100);
  const fatigueColor = ss.fatigue_load > 0.7 ? 'var(--red)' : ss.fatigue_load > 0.4 ? 'var(--yellow)' : 'var(--green)';
  return (
    <div className="sen-subsys-row">
      <span className="sen-subsys-name" style={{ color: fatigued ? 'var(--red)' : undefined }}>{ss.id}</span>
      <div className="sen-bar-track" title={`energy: ${ss.energy}`}>
        <div className="sen-bar-fill sen-energy-fill" style={{ width: `${energyPct}%` }} />
      </div>
      <span className="sen-subsys-val">{ss.energy}</span>
      <div className="sen-bar-track" title={`fatigue: ${(ss.fatigue_load * 100).toFixed(0)}%`}>
        <div className="sen-bar-fill" style={{ width: `${fatiguePct}%`, background: fatigueColor }} />
      </div>
      <span className="sen-subsys-val">{(ss.fatigue_load * 100).toFixed(0)}%</span>
    </div>
  );
}
