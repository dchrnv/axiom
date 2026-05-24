import { useEngineStore } from '../store/engine';

export function Traces() {
  const { snapshot } = useEngineStore();
  if (!snapshot) return <div className="waiting">Waiting for snapshot…</div>;

  const { top_traces, tension_traces, impulses, traces_count, tension_count, skills_count } = snapshot;
  const maxWeight = top_traces.length > 0 ? Math.max(...top_traces.map((t) => t.weight), 0.001) : 0.001;

  return (
    <div className="traces-view">
      {/* Impulses summary */}
      <section className="card">
        <h2>Impulses &amp; Memory</h2>
        <div className="metrics-row">
          <Metric label="Total traces"  value={traces_count.toLocaleString()} />
          <Metric label="Tension"       value={tension_count.toString()} />
          <Metric label="Skills"        value={skills_count.toString()} />
          <Metric label="Goal impulses" value={impulses.goal_count.toString()} />
          <Metric label="Curiosity"     value={impulses.curiosity_count.toString()} />
        </div>
      </section>

      {/* Top traces */}
      <section className="card">
        <h2>Experience Traces — top {top_traces.length} by weight</h2>
        {top_traces.length === 0 ? (
          <div className="waiting" style={{ height: 60 }}>No traces yet</div>
        ) : (
          <div className="traces-table-wrap">
            <table className="traces-table">
              <thead>
                <tr>
                  <th>#</th>
                  <th>Weight</th>
                  <th>Bar</th>
                  <th>tmp/mss/val</th>
                  <th>Position</th>
                  <th>Age</th>
                  <th>Success</th>
                  <th>Hash</th>
                </tr>
              </thead>
              <tbody>
                {top_traces.map((t, i) => (
                  <tr key={i}>
                    <td className="trace-rank">{i + 1}</td>
                    <td className="trace-weight">{t.weight.toFixed(4)}</td>
                    <td>
                      <div className="trace-weight-bar">
                        <div
                          className="trace-weight-fill"
                          style={{ width: `${(t.weight / maxWeight) * 100}%` }}
                        />
                      </div>
                    </td>
                    <td className="trace-tup">{t.temperature}/{t.mass}/{t.valence}</td>
                    <td className="trace-pos">({t.position[0]},{t.position[1]},{t.position[2]})</td>
                    <td className="trace-age">{t.age_ticks.toLocaleString()}</td>
                    <td className="trace-success">{t.success_count}</td>
                    <td className="trace-hash">{`0x${t.pattern_hash.toString(16).padStart(8, '0')}`}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </section>

      {/* Tension traces */}
      {tension_traces.length > 0 && (
        <section className="card">
          <h2>Tension Traces ({tension_traces.length})</h2>
          <div className="tension-list">
            {tension_traces.map((t, i) => (
              <div key={i} className="tension-item">
                <span className="tension-rank">{i + 1}</span>
                <span className="tension-temp">temp {t.temperature}</span>
                <span className="tension-age">{t.age_ticks.toLocaleString()} ticks</span>
                <div
                  className="tension-temp-bar"
                  style={{
                    width: `${Math.round((t.temperature / 255) * 80)}px`,
                    background: t.temperature > 200 ? 'var(--red)' : 'var(--yellow)',
                  }}
                />
              </div>
            ))}
          </div>
        </section>
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
