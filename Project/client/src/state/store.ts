// Project/client/src/state/store.ts
import { create } from "zustand";

export type ConnStatus = "disconnected" | "connecting" | "open";

type State = {
  // connection
  status: ConnStatus;

  // server info
  serverVersion?: string;

  // me + match
  displayName: string;
  matchId: string | null;

  // dev logs (handy while prototyping)
  logs: string[];
};

type Actions = {
  setStatus: (s: ConnStatus) => void;
  setServerVersion: (v: string) => void;
  setDisplayName: (name: string) => void;
  setMatchId: (id: string | null) => void;
  log: (line: string) => void;
  reset: () => void;
};

export const useStore = create<State & Actions>((set) => ({
  status: "disconnected",
  serverVersion: undefined,
  displayName: "",       
  matchId: null,
  logs: [],

  setStatus: (s) => set({ status: s }),
  setServerVersion: (v) => set({ serverVersion: v }),
  setDisplayName: (name) => set({ displayName: name }),
  setMatchId: (id) => set({ matchId: id }),
  log: (line) => set((st) => ({ logs: [...st.logs, line] })),
  reset: () =>
    set({
      status: "disconnected",
      serverVersion: undefined,
      matchId: null,
      logs: [],
    }),
}));