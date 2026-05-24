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

interface EngineStore {
  snapshot: SystemSnapshot | null;
  connected: boolean;
  feed: FeedMessage[];
  // Rolling history of over_domain.layer_activations (last HISTORY_MAX snapshots)
  layerHistory: number[][];

  setSnapshot: (s: SystemSnapshot) => void;
  setConnected: (c: boolean) => void;
  addFeedMessage: (m: Omit<FeedMessage, 'id'>) => void;
}

export const useEngineStore = create<EngineStore>((set) => ({
  snapshot: null,
  connected: false,
  feed: [],
  layerHistory: [],

  setSnapshot: (snapshot) =>
    set((state) => {
      const activations = snapshot.over_domain.layer_activations;
      const layerHistory = [...state.layerHistory, activations].slice(-HISTORY_MAX);
      return { snapshot, layerHistory };
    }),

  setConnected: (connected) => set({ connected }),

  addFeedMessage: (m) =>
    set((state) => ({
      feed: [...state.feed.slice(-199), { ...m, id: ++msgIdCounter }],
    })),
}));
