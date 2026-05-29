import { create } from 'zustand';
import { SystemSnapshot } from '../ws/protocol';

export interface FeedMessage {
  id: number;
  kind: 'user' | 'frame' | 'dream' | 'alert';
  text: string;
  tick?: number;
}

const HISTORY_MAX = 120;
let msgIdCounter = 0;

export interface MetricPoint {
  tick: number;
  hz: number;
  tick_ns: number;
  tokens: number;
  traces: number;
  tension: number;
  fatigue: number;
}

interface EngineStore {
  snapshot: SystemSnapshot | null;
  connected: boolean;
  feed: FeedMessage[];
  layerHistory: number[][];
  metricHistory: MetricPoint[];

  setSnapshot: (s: SystemSnapshot) => void;
  setConnected: (c: boolean) => void;
  addFeedMessage: (m: Omit<FeedMessage, 'id'>) => void;
}

export const useEngineStore = create<EngineStore>((set) => ({
  snapshot: null,
  connected: false,
  feed: [],
  layerHistory: [],
  metricHistory: [],

  setSnapshot: (snapshot) =>
    set((state) => {
      const activations = snapshot.over_domain.layer_activations;
      const layerHistory = [...state.layerHistory, activations].slice(-HISTORY_MAX);

      const point: MetricPoint = {
        tick:    snapshot.current_tick,
        hz:      snapshot.perf.actual_hz,
        tick_ns: snapshot.perf.tick_ns_avg,
        tokens:  snapshot.over_domain.total_tokens,
        traces:  snapshot.traces_count,
        tension: snapshot.tension_count,
        fatigue: snapshot.fatigue.current * 100,
      };
      const metricHistory = [...state.metricHistory, point].slice(-HISTORY_MAX);

      return { snapshot, layerHistory, metricHistory };
    }),

  setConnected: (connected) => set({ connected }),

  addFeedMessage: (m) =>
    set((state) => ({
      feed: [...state.feed.slice(-199), { ...m, id: ++msgIdCounter }],
    })),
}));
