import { useEngineStore } from '../store/engine';

const LAYER_COLORS = [
  '#5b8dee', '#4caf72', '#d4a32a', '#e05c5c',
  '#a05ce0', '#5cd4d4', '#e07a5c', '#8dee8d',
];

export function Domains() {
  const { snapshot } = useEngineStore();
  if (!snapshot) return <div className="waiting">Waiting for snapshot…</div>;

  return (
    <div className="domains-view">
      <section className="card">
        <h2>Domains ({snapshot.domains.length})</h2>
        <div className="domain-grid-full">
          {snapshot.domains.map((d) => {
            const tempPct = Math.round((d.temperature_avg / 255) * 100);
            const tempColor = d.temperature_avg > 200 ? 'var(--red)'
              : d.temperature_avg > 128 ? 'var(--yellow)'
              : 'var(--green)';
            return (
              <div key={d.id} className="domain-card-full">
                <div className="domain-card-header">
                  <span className="domain-name-full">{d.name}</span>
                  <span className="domain-id">#{d.id}</span>
                </div>
                <div className="domain-card-stats">
                  <StatRow label="tokens"  value={d.token_count.toLocaleString()} />
                  <StatRow label="conns"   value={d.connection_count.toLocaleString()} />
                  <StatRow label="temp"    value={`${d.temperature_avg} (${tempPct}%)`} color={tempColor} />
                  <StatRow label="activity" value={d.recent_activity.toLocaleString()} />
                  <StatRow label="capacity" value={d.config_summary.capacity.toLocaleString()} />
                </div>
                <div className="domain-layer-bar-wrap">
                  <LayerBar activations={d.layer_activations} />
                </div>
              </div>
            );
          })}
        </div>
      </section>
    </div>
  );
}

function StatRow({ label, value, color }: { label: string; value: string; color?: string }) {
  return (
    <div className="domain-stat-row">
      <span className="domain-stat-label">{label}</span>
      <span className="domain-stat-value" style={color ? { color } : undefined}>{value}</span>
    </div>
  );
}

function LayerBar({ activations }: { activations: number[] }) {
  const max = Math.max(...activations, 1);
  return (
    <div className="layer-bar">
      {activations.map((v, i) => (
        <div
          key={i}
          className="layer-segment"
          style={{ height: `${Math.round((v / max) * 100)}%`, background: LAYER_COLORS[i] }}
          title={`L${i + 1}: ${v}`}
        />
      ))}
    </div>
  );
}
