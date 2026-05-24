import { useEngineStore } from '../store/engine';

const LAYER_COLORS = [
  '#5b8dee', '#4caf72', '#d4a32a', '#e05c5c',
  '#a05ce0', '#5cd4d4', '#e07a5c', '#8dee8d',
];

function Sparkline({
  data,
  width = 160,
  height = 36,
  color = '#5b8dee',
}: {
  data: number[];
  width?: number;
  height?: number;
  color?: string;
}) {
  if (data.length < 2) return <svg width={width} height={height} />;
  const max = Math.max(...data, 1);
  const min = Math.min(...data);
  const range = max - min || 1;
  const pts = data
    .map((v, i) => {
      const x = (i / (data.length - 1)) * width;
      const y = height - ((v - min) / range) * (height - 2) - 1;
      return `${x.toFixed(1)},${y.toFixed(1)}`;
    })
    .join(' ');
  return (
    <svg width={width} height={height}>
      <polyline points={pts} fill="none" stroke={color} strokeWidth={1.5} strokeLinejoin="round" />
    </svg>
  );
}

export function Patterns() {
  const { snapshot, layerHistory } = useEngineStore();

  if (!snapshot) return <div className="waiting">Waiting for snapshot…</div>;

  // Transpose: layerHistory[t][l] → perLayer[l][t]
  const perLayer: number[][] = Array.from({ length: 8 }, (_, l) =>
    layerHistory.map((row) => row[l] ?? 0)
  );

  const current = snapshot.over_domain.layer_activations;
  const maxCurrent = Math.max(...current, 1);

  return (
    <div className="patterns">
      {/* Current activation bar */}
      <section className="card">
        <h2>Current Layer Activations (over-domain)</h2>
        <div className="layer-current">
          {current.map((v, i) => (
            <div key={i} className="layer-current-item">
              <span className="layer-label">L{i + 1}</span>
              <div className="layer-track">
                <div
                  className="layer-fill"
                  style={{
                    width: `${Math.round((v / maxCurrent) * 100)}%`,
                    background: LAYER_COLORS[i],
                  }}
                />
              </div>
              <span className="layer-val">{v}</span>
            </div>
          ))}
        </div>
      </section>

      {/* Sparklines per layer */}
      <section className="card">
        <h2>Layer History (last {layerHistory.length} snapshots)</h2>
        {layerHistory.length < 2 ? (
          <div className="waiting" style={{ height: 60 }}>Collecting data…</div>
        ) : (
          <div className="sparkline-grid">
            {perLayer.map((data, i) => (
              <div key={i} className="sparkline-item">
                <span className="sparkline-label" style={{ color: LAYER_COLORS[i] }}>
                  L{i + 1}
                </span>
                <Sparkline data={data} color={LAYER_COLORS[i]} width={180} height={40} />
                <span className="sparkline-val">{data[data.length - 1] ?? 0}</span>
              </div>
            ))}
          </div>
        )}
      </section>

      {/* Per-domain sparklines */}
      <section className="card">
        <h2>Domain Activity</h2>
        <div className="domain-grid">
          {snapshot.domains.map((d) => (
            <div key={d.id} className="domain-card">
              <div className="domain-name">{d.name}</div>
              <div className="domain-stats">
                <span>tokens: {d.token_count}</span>
                <span>temp: {d.temperature_avg}</span>
                <span>activity: {d.recent_activity}</span>
              </div>
              <LayerBar activations={d.layer_activations} />
            </div>
          ))}
        </div>
      </section>
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
