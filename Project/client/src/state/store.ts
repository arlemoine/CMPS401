// client/src/state/store.ts
import { create } from "zustand";

interface ChatMessage {
  player_name: string;
  chat_message: string;
  time: string;
}

type GameType = "tictactoe" | "rockpaperscissors" | null;

interface Store {
  playerName: string;
  setPlayerName: (name: string) => void;

  gameId: string | null;
  setGameId: (id: string | null) => void;

  gameType: GameType;
  setGameType: (type: GameType) => void;

  players: string[];
  addPlayer: (player: string) => void;
  removePlayer: (player: string) => void;
  setPlayers: (players: string[]) => void;

  chatMessages: ChatMessage[];
  addChatMessage: (msg: ChatMessage) => void;
  clearChatMessages: () => void;

  board: number[][];
  setBoard: (board: number[][]) => void;

  whosTurn: string;
  setWhosTurn: (turn: string) => void;

  status: string;
  setStatus: (status: string) => void;
}

export const useStore = create<Store>((set) => ({
  playerName: sessionStorage.getItem("ttt_playerName") || "",
  setPlayerName: (name) => {
    sessionStorage.setItem("ttt_playerName", name);
    set({ playerName: name });
  },

  gameId: null,
  setGameId: (id) => set({ gameId: id }),

  gameType: null,
  setGameType: (type) => set({ gameType: type }),

  players: [],
  addPlayer: (player) =>
    set((state) =>
      state.players.includes(player)
        ? state
        : { players: [...state.players, player] }
    ),
  removePlayer: (player) =>
    set((state) => ({
      players: state.players.filter((p) => p !== player),
    })),
  setPlayers: (players) => set({ players }),

  chatMessages: [],
  addChatMessage: (msg) =>
    set((state) => ({ chatMessages: [...state.chatMessages, msg] })),
  clearChatMessages: () => set({ chatMessages: [] }),

  board: [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
  ],
  setBoard: (board) => set({ board }),

  whosTurn: "",
  setWhosTurn: (turn) => set({ whosTurn: turn }),

  status: "waiting",
  setStatus: (status) => set({ status }),
}));