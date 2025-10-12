// Project/client/src/state/store.ts
import { create } from "zustand";
import type { Player } from "../api/ws";

interface GameStore {
  // User identity
  displayName: string;
  setDisplayName: (name: string) => void;

  // Current match
  matchId: string;
  setMatchId: (id: string) => void;

  // Match state
  players: Player[];
  setPlayers: (players: Player[]) => void;

  matchStatus: string; // "WAITING", "IN_PROGRESS", "FINISHED"
  setMatchStatus: (status: string) => void;

  // Current player info
  me: Player | null;
  setMe: (player: Player | null) => void;
}

export const useStore = create<GameStore>((set) => ({
  displayName: "",
  setDisplayName: (name) => set({ displayName: name }),

  matchId: "",
  setMatchId: (id) => set({ matchId: id }),

  players: [],
  setPlayers: (players) => set({ players }),

  matchStatus: "WAITING",
  setMatchStatus: (status) => set({ matchStatus: status }),

  me: null,
  setMe: (player) => set({ me: player }),
}));