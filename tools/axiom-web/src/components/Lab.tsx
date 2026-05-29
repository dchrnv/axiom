import { useCallback, useEffect, useRef, useState } from 'react';

type JobStatus = 'idle' | 'running' | 'done' | 'failed';

interface LabStatus {
  status: JobStatus;
  job: string | null;
  exit_code: number | null;
}

interface ObsProgress {
  tick: number;
  total: number;
  pct: number;
  elapsed: number;
  eta: number;
}

const JOBS = [
  { id: 'obs',          label: 'OBS',           desc: 'Corpus run (axiom-observe)' },
  { id: 'bench_hot',    label: 'Hot Bench',      desc: 'hot_path_regression' },
  { id: 'bench_od',     label: 'Over-Domain',    desc: 'over_domain_bench' },
  { id: 'bench_stress', label: 'Stress Bench',   desc: 'stress_bench' },
  { id: 'test',         label: 'Tests',          desc: 'cargo test --workspace' },
  { id: 'showcase',     label: 'Full Showcase',  desc: 'OBS + all benches → SHOWCASE.md' },
] as const;

const MAX_LOG_LINES = 2000;

// Parse [observe] N/M (P%) — elapsed=Xs eta=Ys
function parseObsProgress(line: string): ObsProgress | null {
  const m = line.match(/\[observe\]\s+(\d+)\/(\d+)\s+\((\d+(?:\.\d+)?)%\).*?(\d+(?:\.\d+)?)s elapsed.*?~?(\d+(?:\.\d+)?)s left/);
  if (!m) return null;
  return {
    tick:    parseInt(m[1]),
    total:   parseInt(m[2]),
    pct:     parseFloat(m[3]),
    elapsed: parseFloat(m[4]),
    eta:     parseFloat(m[5]),
  };
}

export function Lab() {
  const [status, setStatus] = useState<LabStatus>({ status: 'idle', job: null, exit_code: null });
  const [logs, setLogs] = useState<string[]>([]);
  const [progress, setProgress] = useState<ObsProgress | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const logRef = useRef<HTMLPreElement>(null);
  const autoScrollRef = useRef(true);

  // Poll status every 3s
  useEffect(() => {
    const poll = async () => {
      try {
        const res = await fetch('/api/lab/status');
        if (res.ok) setStatus(await res.json());
      } catch { /* ignore */ }
    };
    poll();
    const id = setInterval(poll, 3000);
    return () => clearInterval(id);
  }, []);

  // Connect log WebSocket
  useEffect(() => {
    const proto = location.protocol === 'https:' ? 'wss' : 'ws';
    const ws = new WebSocket(`${proto}://${location.host}/api/lab/ws/log`);
    wsRef.current = ws;

    ws.onmessage = (e) => {
      const line = e.data as string;
      setLogs((prev) => {
        const next = [...prev, line];
        return next.length > MAX_LOG_LINES ? next.slice(-MAX_LOG_LINES) : next;
      });
      const p = parseObsProgress(line);
      if (p) setProgress(p);
      if (line.includes('[lab] done') || line.includes('[lab] stopped') || line.includes('[lab] error')) {
        setProgress(null);
      }
    };

    ws.onclose = () => { wsRef.current = null; };
    return () => ws.close();
  }, []);

  // Auto-scroll log
  useEffect(() => {
    if (autoScrollRef.current && logRef.current) {
      logRef.current.scrollTop = logRef.current.scrollHeight;
    }
  }, [logs]);

  const runJob = useCallback(async (jobId: string) => {
    setLogs([]);
    setProgress(null);
    await fetch('/api/lab/run', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ job: jobId }),
    });
    setStatus({ status: 'running', job: jobId, exit_code: null });
  }, []);

  const stopJob = useCallback(async () => {
    await fetch('/api/lab/stop', { method: 'POST' });
  }, []);

  const clearLogs = useCallback(() => setLogs([]), []);

  const isRunning = status.status === 'running';

  return (
    <div className="lab">
      {/* ── Job buttons ──────────────────────────────────────────────────── */}
      <section className="card lab-jobs">
        <h2>Run</h2>
        <div className="lab-job-grid">
          {JOBS.map((j) => (
            <button
              key={j.id}
              className={`lab-job-btn ${isRunning && status.job === j.id ? 'lab-job-active' : ''}`}
              disabled={isRunning && status.job !== j.id}
              onClick={() => runJob(j.id)}
              title={j.desc}
            >
              <span className="lab-job-label">{j.label}</span>
              <span className="lab-job-desc">{j.desc}</span>
            </button>
          ))}
        </div>
      </section>

      {/* ── Status + progress ─────────────────────────────────────────────── */}
      <section className="card lab-status-row">
        <StatusBadge status={status.status} job={status.job} />
        {isRunning && (
          <button className="lab-stop-btn" onClick={stopJob}>■ Stop</button>
        )}
        {progress && (
          <div className="lab-progress-wrap">
            <div className="lab-progress-bar-track">
              <div
                className="lab-progress-bar-fill"
                style={{ width: `${progress.pct}%` }}
              />
            </div>
            <span className="lab-progress-label">
              {progress.tick.toLocaleString()} / {progress.total.toLocaleString()}
              &nbsp;&nbsp;{progress.pct.toFixed(1)}%
              &nbsp;&nbsp;elapsed {fmtSec(progress.elapsed)}
              &nbsp;&nbsp;eta ~{fmtSec(progress.eta)}
            </span>
          </div>
        )}
      </section>

      {/* ── Log monitor ──────────────────────────────────────────────────── */}
      <section className="card lab-log-card">
        <div className="lab-log-header">
          <h2>Log</h2>
          <label className="lab-autoscroll-label">
            <input
              type="checkbox"
              checked={autoScrollRef.current}
              onChange={(e) => { autoScrollRef.current = e.target.checked; }}
            />
            &nbsp;auto-scroll
          </label>
          <button className="lab-clear-btn" onClick={clearLogs}>Clear</button>
        </div>
        <pre
          ref={logRef}
          className="lab-log"
          onScroll={() => {
            if (!logRef.current) return;
            const { scrollTop, scrollHeight, clientHeight } = logRef.current;
            autoScrollRef.current = scrollHeight - scrollTop - clientHeight < 40;
          }}
        >
          {logs.length === 0
            ? <span className="lab-log-empty">No output yet. Start a job above.</span>
            : logs.map((line, i) => (
              <span key={i} className={`lab-log-line ${colorClass(line)}`}>{line}{'\n'}</span>
            ))
          }
        </pre>
      </section>
    </div>
  );
}

function StatusBadge({ status, job }: { status: JobStatus; job: string | null }) {
  const labels: Record<JobStatus, string> = {
    idle:    '○ Idle',
    running: '● Running',
    done:    '✓ Done',
    failed:  '✗ Failed',
  };
  return (
    <span className={`lab-status lab-status-${status}`}>
      {labels[status]}{job ? ` — ${job}` : ''}
    </span>
  );
}

function colorClass(line: string): string {
  if (line.includes('[observe]') && line.includes('%')) return 'log-progress';
  if (line.includes('[lab] done') || line.includes('test result: ok')) return 'log-ok';
  if (line.includes('[lab] failed') || line.includes('FAILED') || line.includes('error[')) return 'log-err';
  if (line.includes('time:') || line.includes('thrpt:')) return 'log-bench';
  if (line.includes('[observe]')) return 'log-obs';
  return '';
}

function fmtSec(s: number): string {
  if (s < 60) return `${s.toFixed(0)}s`;
  if (s < 3600) return `${Math.floor(s / 60)}m${Math.floor(s % 60)}s`;
  return `${Math.floor(s / 3600)}h${Math.floor((s % 3600) / 60)}m`;
}
