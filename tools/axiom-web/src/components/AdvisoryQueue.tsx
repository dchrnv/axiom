import { useState } from 'react';
import type { PendingAdvisorySnapshot } from '../ws/protocol';

const ADVISORY_TYPES: Record<number, string> = {
  0: 'DepthHint',
  1: 'OctantCorr',
  2: 'NarrShift',
  3: 'SubsysCorr',
  4: 'EmergentCand',
};

const PENDING_TTL = 1000;

export function AdvisoryQueue({
  advisories,
  currentEvent,
}: {
  advisories: PendingAdvisorySnapshot[];
  currentEvent: number;
}) {
  const [busy, setBusy] = useState<Set<number>>(new Set());

  async function confirm(id: number) {
    setBusy((s) => new Set(s).add(id));
    await fetch(`/api/advisory/confirm/${id}`, { method: 'POST' });
    setBusy((s) => { const n = new Set(s); n.delete(id); return n; });
  }

  async function reject(id: number) {
    setBusy((s) => new Set(s).add(id));
    await fetch(`/api/advisory/reject/${id}`, { method: 'POST' });
    setBusy((s) => { const n = new Set(s); n.delete(id); return n; });
  }

  if (advisories.length === 0) return null;

  return (
    <section className="card advisory-queue">
      <h2>Pending Advisories ({advisories.length})</h2>
      <table>
        <thead>
          <tr>
            <th>Type</th>
            <th>Subject</th>
            <th>Conf</th>
            <th>Label</th>
            <th>Age / TTL</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {advisories.map((a) => {
            const age = currentEvent - a.queued_at_event;
            const ttlPct = Math.min(100, Math.round((age / PENDING_TTL) * 100));
            const isBusy = busy.has(Number(a.advisory_id));
            return (
              <tr key={String(a.advisory_id)}>
                <td className="advisory-type">{ADVISORY_TYPES[a.advisory_type] ?? `T${a.advisory_type}`}</td>
                <td>#{a.subject_id}</td>
                <td>{a.confidence.toFixed(2)}</td>
                <td className="advisory-label">{a.label}</td>
                <td>
                  <div className="ttl-bar">
                    <div className="ttl-fill" style={{ width: `${ttlPct}%` }} data-critical={ttlPct > 80 ? '' : undefined} />
                    <span className="ttl-label">{age}ev</span>
                  </div>
                </td>
                <td className="advisory-actions">
                  <button className="btn-confirm" disabled={isBusy} onClick={() => confirm(Number(a.advisory_id))} title="Confirm">✓</button>
                  <button className="btn-reject"  disabled={isBusy} onClick={() => reject(Number(a.advisory_id))}  title="Reject">✗</button>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </section>
  );
}
