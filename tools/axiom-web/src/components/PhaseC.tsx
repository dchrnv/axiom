import { useEngineStore } from '../store/engine';
import { AdvisoryQueue } from './AdvisoryQueue';

const OCTANT_NAMES = [
  'CreativeAffirmation',
  'EcstaticAffirmation',
  'HeroicFatal',
  'DestructiveActivating',
  'IdealizedConsoling',
  'PassiveSentimental',
  'FormalDenying',
  'SelfDestructiveApathic',
];

const SUBSYSTEM_NAMES = ['Writing', 'Mathematics', 'Philosophy', 'Code', 'Analysis', 'Unknown'];

export function PhaseC() {
  const { snapshot } = useEngineStore();

  if (!snapshot?.phase_c) {
    return <div className="waiting">No Phase C data yet.</div>;
  }

  const pc = snapshot.phase_c;
  const dominantOctant =
    pc.dominant_octant != null ? OCTANT_NAMES[pc.dominant_octant] ?? `#${pc.dominant_octant}` : '—';
  const dominantSubsystem =
    pc.dominant_subsystem != null ? SUBSYSTEM_NAMES[pc.dominant_subsystem] ?? `#${pc.dominant_subsystem}` : '—';

  const maxDepth = Math.max(...pc.octant_depth_avg, 1);

  return (
    <div className="phase-c">
      {/* Dominant */}
      <section className="card">
        <h2>Axial State</h2>
        <div className="phase-c-dominant">
          <div className="phase-c-dominant-item">
            <span className="phase-c-dominant-label">Dominant Octant</span>
            <span className="phase-c-dominant-value">{dominantOctant}</span>
          </div>
          <div className="phase-c-dominant-item">
            <span className="phase-c-dominant-label">Dominant Subsystem</span>
            <span className="phase-c-dominant-value">{dominantSubsystem}</span>
          </div>
          <div className="phase-c-dominant-item">
            <span className="phase-c-dominant-label">Emergent Candidates</span>
            <span className="phase-c-dominant-value">{pc.pending_emergent_count}</span>
          </div>
        </div>
      </section>

      {/* Octant depth bars */}
      <section className="card">
        <h2>Octant Depth</h2>
        <div className="octant-bars">
          {pc.octant_depth_avg.map((depth, i) => (
            <div key={i} className={`octant-bar-item ${pc.dominant_octant === i ? 'octant-bar-dominant' : ''}`}>
              <div className="octant-bar-name">{OCTANT_NAMES[i]}</div>
              <div className="octant-bar-track">
                <div className="octant-bar-fill" style={{ width: `${Math.round((depth / maxDepth) * 100)}%` }} />
              </div>
              <div className="octant-bar-val">{depth.toLocaleString()}</div>
            </div>
          ))}
        </div>
      </section>

      {/* Emergent candidates */}
      {pc.emergent_candidates.length > 0 && (
        <section className="card">
          <h2>Emergent Candidates ({pc.emergent_candidates.length})</h2>
          <table className="emergent-table">
            <thead>
              <tr>
                <th>Sutra ID</th>
                <th>Octant</th>
                <th>Initial Depth</th>
                <th></th>
              </tr>
            </thead>
            <tbody>
              {pc.emergent_candidates.map((c) => (
                <tr key={c.sutra_id}>
                  <td>#{c.sutra_id}</td>
                  <td>{OCTANT_NAMES[c.discovery_octant] ?? c.discovery_octant}</td>
                  <td>{c.initial_depth}</td>
                  <td>
                    <button
                      className="btn-approve"
                      onClick={() =>
                        fetch(`/api/text/submit`, {
                          method: 'POST',
                          headers: { 'Content-Type': 'application/json' },
                          body: JSON.stringify({ text: `:approve ${c.sutra_id}` }),
                        })
                      }
                    >
                      Approve
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </section>
      )}

      {/* Advisory Queue */}
      {pc.pending_advisories.length > 0 && (
        <AdvisoryQueue
          advisories={pc.pending_advisories}
          currentEvent={snapshot.current_event}
        />
      )}
    </div>
  );
}
