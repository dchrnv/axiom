import { useCallback, useEffect, useRef, useState } from 'react';

type JobStatus = 'idle' | 'running' | 'paused' | 'done' | 'failed';

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

interface HistoryEntry {
  id: string;
  job: string;
  status: 'done' | 'failed';
  ts: number;       // Date.now()
  duration: number; // seconds
  summary: string;
}

const JOBS = [
  { id: 'obs_quick',    label: 'OBS Quick',     desc: 'corpus_mixed · 60K тиков' },
  { id: 'obs',          label: 'OBS Full',      desc: 'corpus_large · 1M тиков' },
  { id: 'bench_hot',    label: 'Hot Bench',      desc: 'hot_path_regression' },
  { id: 'bench_od',     label: 'Over-Domain',    desc: 'over_domain_bench' },
  { id: 'bench_stress', label: 'Stress Bench',   desc: 'stress_bench' },
  { id: 'test',         label: 'Tests',          desc: 'cargo test --workspace' },
  { id: 'showcase',     label: 'Full Showcase',  desc: 'OBS + all benches → SHOWCASE.md' },
] as const;

const MAX_LOG_LINES = 2000;
const HISTORY_KEY = 'axiom_lab_history';
const MAX_HISTORY = 10;

function parseObsProgress(line: string): ObsProgress | null {
  const m = line.match(/\[observe(?:\/shard\d+)?\]\s+(\d+)\/(\d+)\s+\((\d+(?:\.\d+)?)%\).*?(\d+(?:\.\d+)?)s elapsed.*?~?(\d+(?:\.\d+)?)s left/);
  if (!m) return null;
  return { tick: parseInt(m[1]), total: parseInt(m[2]), pct: parseFloat(m[3]), elapsed: parseFloat(m[4]), eta: parseFloat(m[5]) };
}

function findLast(arr: string[], pred: (l: string) => boolean): string | undefined {
  for (let i = arr.length - 1; i >= 0; i--) { if (pred(arr[i])) return arr[i]; }
  return undefined;
}

function extractSummary(job: string | null, logs: string[]): string {
  if (!job) return '';
  if (job === 'obs' || job === 'obs_quick' || job === 'showcase') {
    const doneLine = findLast(logs, l => l.includes('[observe') && l.includes('ticks/sec'));
    if (doneLine) return doneLine.replace(/^\[observe[^\]]*\]\s*/, '');
    const reportLine = findLast(logs, l => l.includes('report written'));
    if (reportLine) return reportLine;
  }
  if (job.startsWith('bench')) {
    const lines = logs.filter(l => l.includes('time:') || l.includes('thrpt:'));
    if (lines.length) return lines.slice(-3).join(' | ');
  }
  if (job === 'test') {
    const resultLine = findLast(logs, l => l.includes('test result'));
    if (resultLine) return resultLine.trim();
  }
  return findLast(logs, l => l.includes('[lab] done')) ?? '';
}

function loadHistory(): HistoryEntry[] {
  try { return JSON.parse(localStorage.getItem(HISTORY_KEY) ?? '[]'); } catch { return []; }
}

function saveHistory(h: HistoryEntry[]) {
  localStorage.setItem(HISTORY_KEY, JSON.stringify(h.slice(0, MAX_HISTORY)));
}

function fmtDuration(s: number): string {
  if (s < 60) return `${s.toFixed(0)}s`;
  if (s < 3600) return `${Math.floor(s / 60)}m${Math.floor(s % 60)}s`;
  return `${Math.floor(s / 3600)}h${Math.floor((s % 3600) / 60)}m`;
}

function fmtTime(ts: number): string {
  const d = new Date(ts);
  return `${d.toLocaleDateString()} ${d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
}

// ── Results panel ────────────────────────────────────────────────────────────

function ResultsPanel({ job, logs, status }: { job: string | null; logs: string[]; status: JobStatus }) {
  if (status !== 'done' && status !== 'failed') return null;
  if (!logs.length) return null;

  const isFailed = status === 'failed';

  // OBS / showcase results
  if (job === 'obs' || job === 'obs_quick' || job === 'showcase') {
    const doneLine = findLast(logs, l => l.includes('done in') && l.includes('ticks/sec')) ?? '';
    const reportLine = findLast(logs, l => l.includes('snapshots') && l.includes('events')) ?? '';
    return (
      <section className="card lab-results">
        <h2 className={`lab-results-title ${isFailed ? 'lab-res-fail' : 'lab-res-ok'}`}>
          {isFailed ? '✗ Run failed' : '✓ OBS complete'}
        </h2>
        {doneLine && <div className="lab-res-line">{doneLine}</div>}
        {reportLine && <div className="lab-res-line">{reportLine}</div>}
        <div className="lab-res-hint">Full report: <code>showcase/obs_out/report.md</code></div>
      </section>
    );
  }

  // Benchmark results
  if (job?.startsWith('bench')) {
    const timeLines = logs.filter(l => l.includes('time:') || l.includes('thrpt:'));
    return (
      <section className="card lab-results">
        <h2 className={`lab-results-title ${isFailed ? 'lab-res-fail' : 'lab-res-ok'}`}>
          {isFailed ? '✗ Bench failed' : '✓ Bench complete'}
        </h2>
        {timeLines.length > 0 && (
          <div className="lab-res-bench-lines">
            {timeLines.map((l, i) => <div key={i} className="lab-res-bench-line">{l.trim()}</div>)}
          </div>
        )}
        <div className="lab-res-hint">HTML reports: <code>target/criterion/</code></div>
      </section>
    );
  }

  // Test results
  if (job === 'test') {
    const resultLines = logs.filter(l => l.includes('test result'));
    const failed = logs.filter(l => l.includes('FAILED'));
    return (
      <section className="card lab-results">
        <h2 className={`lab-results-title ${isFailed ? 'lab-res-fail' : 'lab-res-ok'}`}>
          {isFailed ? '✗ Tests failed' : '✓ All tests passed'}
        </h2>
        {resultLines.map((l, i) => <div key={i} className="lab-res-line">{l.trim()}</div>)}
        {failed.map((l, i) => <div key={i} className="lab-res-line log-err">{l.trim()}</div>)}
      </section>
    );
  }

  return null;
}

// ── History panel ────────────────────────────────────────────────────────────

function HistoryPanel({ history, onClear }: { history: HistoryEntry[]; onClear: () => void }) {
  if (!history.length) return null;
  return (
    <section className="card lab-history">
      <div className="lab-history-header">
        <h2>History</h2>
        <button className="lab-clear-btn" onClick={onClear}>Clear</button>
      </div>
      <div className="lab-history-list">
        {history.map(h => (
          <div key={h.id} className={`lab-history-item ${h.status === 'failed' ? 'lab-hist-fail' : 'lab-hist-ok'}`}>
            <span className="lab-hist-icon">{h.status === 'done' ? '✓' : '✗'}</span>
            <span className="lab-hist-job">{h.job}</span>
            <span className="lab-hist-dur">{fmtDuration(h.duration)}</span>
            <span className="lab-hist-time">{fmtTime(h.ts)}</span>
            {h.summary && <span className="lab-hist-summary">{h.summary}</span>}
          </div>
        ))}
      </div>
    </section>
  );
}

// ── Main Lab component ───────────────────────────────────────────────────────

export function Lab() {
  const [status, setStatus] = useState<LabStatus>({ status: 'idle', job: null, exit_code: null });
  const [logs, setLogs] = useState<string[]>([]);
  const [progress, setProgress] = useState<ObsProgress | null>(null);
  const [history, setHistory] = useState<HistoryEntry[]>(loadHistory);
  const wsRef = useRef<WebSocket | null>(null);
  const logRef = useRef<HTMLPreElement>(null);
  const autoScrollRef = useRef(true);
  const startTimeRef = useRef<number>(0);
  const prevStatusRef = useRef<JobStatus>('idle');

  // Poll status every 3s
  useEffect(() => {
    const poll = async () => {
      try {
        const res = await fetch('/api/lab/status');
        if (!res.ok) return;
        const s: LabStatus = await res.json();
        setStatus(prev => {
          // Detect completion: running → done/failed
          if (prev.status === 'running' && (s.status === 'done' || s.status === 'failed')) {
            const duration = (Date.now() - startTimeRef.current) / 1000;
            const entry: HistoryEntry = {
              id: `${Date.now()}`,
              job: prev.job ?? s.job ?? '?',
              status: s.status,
              ts: Date.now(),
              duration,
              summary: extractSummary(prev.job, logsSnapshotRef.current),
            };
            setHistory(h => {
              const next = [entry, ...h].slice(0, MAX_HISTORY);
              saveHistory(next);
              return next;
            });
            setProgress(null);
          }
          return s;
        });
      } catch { /* ignore */ }
    };
    poll();
    const id = setInterval(poll, 3000);
    return () => clearInterval(id);
  }, []);

  // Logs snapshot ref for history summary extraction
  const logsSnapshotRef = useRef<string[]>([]);
  useEffect(() => { logsSnapshotRef.current = logs; }, [logs]);

  // Connect log WebSocket
  useEffect(() => {
    const proto = location.protocol === 'https:' ? 'wss' : 'ws';
    const ws = new WebSocket(`${proto}://${location.host}/api/lab/ws/log`);
    wsRef.current = ws;
    ws.onmessage = (e) => {
      const line = e.data as string;
      setLogs(prev => {
        const next = [...prev, line];
        return next.length > MAX_LOG_LINES ? next.slice(-MAX_LOG_LINES) : next;
      });
      const p = parseObsProgress(line);
      if (p) setProgress(p);
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
    startTimeRef.current = Date.now();
    prevStatusRef.current = 'running';
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

  const pauseJob = useCallback(async () => {
    await fetch('/api/lab/pause', { method: 'POST' });
  }, []);

  const resumeJob = useCallback(async () => {
    await fetch('/api/lab/resume', { method: 'POST' });
  }, []);

  const isRunning = status.status === 'running';
  const isPaused = status.status === 'paused';

  return (
    <div className="lab">
      {/* ── Job buttons ────────────────────────────────────────────────── */}
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

      {/* ── Status + progress ──────────────────────────────────────────── */}
      <section className="card lab-status-row">
        <StatusBadge status={status.status} job={status.job} />
        {isRunning && <button className="lab-pause-btn" onClick={pauseJob}>⏸ Pause</button>}
        {isPaused && <button className="lab-resume-btn" onClick={resumeJob}>▶ Resume</button>}
        {(isRunning || isPaused) && <button className="lab-stop-btn" onClick={stopJob}>■ Stop</button>}
        {progress && (
          <div className="lab-progress-wrap">
            <div className="lab-progress-bar-track">
              <div className="lab-progress-bar-fill" style={{ width: `${progress.pct}%` }} />
            </div>
            <span className="lab-progress-label">
              {progress.tick.toLocaleString()} / {progress.total.toLocaleString()}
              &nbsp;&nbsp;{progress.pct.toFixed(1)}%
              &nbsp;&nbsp;{fmtDuration(progress.elapsed)} elapsed
              &nbsp;&nbsp;~{fmtDuration(progress.eta)} left
            </span>
          </div>
        )}
      </section>

      {/* ── Results panel (post-run) ────────────────────────────────────── */}
      <ResultsPanel job={status.job} logs={logs} status={status.status} />

      {/* ── Log monitor ────────────────────────────────────────────────── */}
      <section className="card lab-log-card">
        <div className="lab-log-header">
          <h2>Log</h2>
          <label className="lab-autoscroll-label">
            <input type="checkbox" defaultChecked onChange={(e) => { autoScrollRef.current = e.target.checked; }} />
            &nbsp;auto-scroll
          </label>
          <button className="lab-clear-btn" onClick={() => setLogs([])}>Clear</button>
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
            : logs.map((line, i) => <span key={i} className={`lab-log-line ${colorClass(line)}`}>{line}{'\n'}</span>)
          }
        </pre>
      </section>

      {/* ── Run history ────────────────────────────────────────────────── */}
      <HistoryPanel history={history} onClear={() => { setHistory([]); saveHistory([]); }} />
    </div>
  );
}

function StatusBadge({ status, job }: { status: JobStatus; job: string | null }) {
  const labels: Record<JobStatus, string> = { idle: '○ Idle', running: '● Running', paused: '⏸ Paused', done: '✓ Done', failed: '✗ Failed' };
  return <span className={`lab-status lab-status-${status}`}>{labels[status]}{job ? ` — ${job}` : ''}</span>;
}

function colorClass(line: string): string {
  if (line.includes('[observe') && line.includes('%')) return 'log-progress';
  if (line.includes('[lab] done') || line.includes('test result: ok')) return 'log-ok';
  if (line.includes('[lab] failed') || line.includes('FAILED') || line.includes('error[')) return 'log-err';
  if (line.includes('time:') || line.includes('thrpt:')) return 'log-bench';
  if (line.includes('[observe')) return 'log-obs';
  return '';
}
