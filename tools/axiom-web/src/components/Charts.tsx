import { useEngineStore, MetricPoint } from '../store/engine';

// ── SVG line chart ────────────────────────────────────────────────────────────

interface ChartProps {
  data: number[];
  label: string;
  unit?: string;
  color?: string;
  height?: number;
  format?: (v: number) => string;
}

function LineChart({ data, label, unit = '', color = 'var(--accent)', height = 60, format }: ChartProps) {
  if (data.length < 2) {
    return (
      <div className="chart-card">
        <div className="chart-label">{label}</div>
        <div className="chart-waiting">Waiting for data…</div>
      </div>
    );
  }

  const min = Math.min(...data);
  const max = Math.max(...data);
  const range = max - min || 1;
  const w = 260;
  const h = height;
  const pad = 4;

  const pts = data.map((v, i) => {
    const x = pad + (i / (data.length - 1)) * (w - pad * 2);
    const y = h - pad - ((v - min) / range) * (h - pad * 2);
    return `${x},${y}`;
  }).join(' ');

  const last = data[data.length - 1];
  const fmt = format ?? ((v: number) => v.toFixed(1));

  return (
    <div className="chart-card">
      <div className="chart-header">
        <span className="chart-label">{label}</span>
        <span className="chart-value" style={{ color }}>{fmt(last)}{unit}</span>
      </div>
      <svg width={w} height={h} className="chart-svg">
        {/* Grid lines */}
        {[0.25, 0.5, 0.75].map(f => (
          <line key={f}
            x1={pad} y1={pad + (1 - f) * (h - pad * 2)}
            x2={w - pad} y2={pad + (1 - f) * (h - pad * 2)}
            stroke="var(--border)" strokeWidth="0.5" />
        ))}
        {/* Area fill */}
        <polyline
          points={`${pad},${h - pad} ${pts} ${w - pad},${h - pad}`}
          fill={color}
          fillOpacity="0.08"
          stroke="none"
        />
        {/* Line */}
        <polyline points={pts} fill="none" stroke={color} strokeWidth="1.5" strokeLinejoin="round" />
        {/* Last point dot */}
        {(() => {
          const last_x = pad + ((data.length - 1) / (data.length - 1)) * (w - pad * 2);
          const last_y = h - pad - ((last - min) / range) * (h - pad * 2);
          return <circle cx={last_x} cy={last_y} r="2.5" fill={color} />;
        })()}
      </svg>
      <div className="chart-range">
        <span>{fmt(min)}{unit}</span>
        <span>{fmt(max)}{unit}</span>
      </div>
    </div>
  );
}

// ── Formatter helpers ─────────────────────────────────────────────────────────

function fmtNs(ns: number): string {
  if (ns < 1_000) return `${ns.toFixed(0)}`;
  if (ns < 1_000_000) return `${(ns / 1_000).toFixed(1)}`;
  return `${(ns / 1_000_000).toFixed(2)}`;
}
function unitNs(ns: number): string {
  if (ns < 1_000) return 'ns';
  if (ns < 1_000_000) return 'µs';
  return 'ms';
}

// ── Charts tab ────────────────────────────────────────────────────────────────

export function Charts() {
  const { metricHistory } = useEngineStore();

  const get = (key: keyof MetricPoint) => metricHistory.map(p => p[key] as number);

  const tick_ns = get('tick_ns');
  const tickUnit = tick_ns.length ? unitNs(tick_ns[tick_ns.length - 1]) : 'µs';

  return (
    <div className="charts-grid">
      <LineChart
        data={get('hz')}
        label="Tick Hz"
        unit=" Hz"
        color="var(--green)"
        format={v => v.toFixed(1)}
      />
      <LineChart
        data={tick_ns}
        label="Avg tick"
        unit={` ${tickUnit}`}
        color="var(--accent)"
        format={fmtNs}
      />
      <LineChart
        data={get('tokens')}
        label="Tokens"
        color="var(--yellow)"
        format={v => v.toFixed(0)}
      />
      <LineChart
        data={get('traces')}
        label="Traces"
        color="#a78bfa"
        format={v => v.toFixed(0)}
      />
      <LineChart
        data={get('tension')}
        label="Tension"
        color="var(--red)"
        format={v => v.toFixed(0)}
      />
      <LineChart
        data={get('fatigue')}
        label="Fatigue"
        unit="%"
        color="var(--yellow)"
        format={v => v.toFixed(1)}
      />
    </div>
  );
}
