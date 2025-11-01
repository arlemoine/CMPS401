// client/src/state/store.ts
import { create } from 'zustand';
import type { Player } from '../api/ws';

interface Store {
  displayName: string;
  setDisplayName: (name: string) => void;
  
  matchId: string | null;
  setMatchId: (id: string | null) => void;
  
  players: Player[];
  setPlayers: (players: Player[]) => void;
  
  matchStatus: string;
  setMatchStatus: (status: string) => void;
  
  me: Player | null;
  setMe: (player: Player | null) => void;
  
  board: (string | null)[];
  setBoard: (board: (string | null)[]) => void;
  
  turn: string | null;
  setTurn: (turn: string | null) => void;
}

export const useStore = create<Store>((set) => ({
  displayName: '',
  setDisplayName: (name) => set({ displayName: name }),
  
  matchId: null,
  setMatchId: (id) => set({ matchId: id }),
  
  players: [],
  setPlayers: (players) => set({ players }),
  
  matchStatus: 'WAITING',
  setMatchStatus: (status) => set({ matchStatus: status }),
  
  me: null,
  setMe: (player) => set({ me: player }),
  
  board: Array(9).fill(null),
  setBoard: (board) => set({ board }),
  
  turn: null,
  setTurn: (turn) => set({ turn }),
}));