// client/src/api/ws.ts
// WebSocket API aligned with Rust backend protocol

// Card type for Uno
export interface UnoCard {
  color: string; // "Red", "Yellow", "Green", "Blue", "Wild"
  rank: string;  // "0"-"9", "Skip", "Reverse", "DrawTwo", "Wild", "WildDrawFour"
}

export type ClientMsg =
  | {
      type: "Echo";
      data: { message: string };
    }
  | {
      type: "GameRoom";
      data: {
        game: string;
        action: "join" | "leave" | "reset";
        player_name: string;
        game_id: string;
      };
    }
  | {
      type: "Chat";
      data: {
        action: "send" | "broadcast";
        game_id: string;
        player_name: string;
        chat_message: string;
        time: string;
      };
    }
  | {
      type: "TicTacToe";
      data: {
        game_id: string;
        whos_turn: string;
        choice: string;
      };
    }
  | {
      type: "RockPaperScissors";
      data: {
        game_id: string;
        player_name: string;
        choice?: string;
      };
    }
  | {
      type: "Uno";
      data: {
        game_id: string;
        player_name: string;
        action: string; // "start", "play_card", "draw_card", "pass_turn", "call_uno", "request_state"
        card?: UnoCard;
        choose_color?: string; // "Red", "Yellow", "Green", "Blue"
        call_uno?: boolean;
      };
    };

export type ServerMsg =
  | { type: "Echo"; data: { message: string } }
  | {
      type: "GameRoom";
      data: {
        game?: string;
        action: string;
        game_id: string;
        player_name?: string;
        players?: string[];
      };
    }
  | {
      type: "Chat";
      data: {
        action: "send" | "broadcast";
        game_id: string;
        player_name: string;
        chat_message: string;
        time: string;
      };
    }
  | {
      type: "TicTacToe";
      data: {
        board?: number[][];
        whos_turn?: string;
        status?: string;
      };
    }
  | {
      type: "RockPaperScissors";
      data: {
        game_id: string;
        player1?: string | null;
        player2?: string | null;
        player1_choice?: string | null;
        player2_choice?: string | null;
        status: string;
        winner?: string | null;
        message?: string | null;
      };
    }
  | {
      type: "Uno";
      data: {
        game_id: string;
        players?: string[] | null;
        current_idx?: number | null;
        direction?: number | null;
        top_discard?: UnoCard | null;
        chosen_color?: string | null;
        pending_draw?: number | null;
        public_counts?: number[] | null;
        hand?: UnoCard[] | null;
        winner?: string | null;
      };
    };

const WS_URL = import.meta.env.VITE_WS_URL as string;

export type Unsubscribe = () => void;
export type WSHandle = {
  readonly socket: WebSocket | null;
  connect: () => void;
  send: (msg: ClientMsg) => void;
  close: () => void;
  onMessage: (handler: (msg: ServerMsg) => void) => Unsubscribe;
  onOpen: (handler: () => void) => Unsubscribe;
  onClose: (handler: (code: number, reason: string) => void) => Unsubscribe;
};

declare global {
  interface Window {
    __TTT_WS__?: ReturnType<typeof createWS>;
  }
}

function createWS(): WSHandle {
  let socket: WebSocket | null = null;
  let connecting = false;
  let reconnectAttempts = 0;
  const MAX_RECONNECT_ATTEMPTS = 5;
  let outboundQueue: ClientMsg[] = [];

  const messageHandlers = new Set<(msg: ServerMsg) => void>();
  const openHandlers = new Set<() => void>();
  const closeHandlers = new Set<(code: number, reason: string) => void>();

  const isOpen = () => socket?.readyState === WebSocket.OPEN;

  const flush = (s: WebSocket) => {
    for (const m of outboundQueue) {
      try {
        s.send(JSON.stringify(m));
        console.log("[ws] => (queued)", m);
      } catch (e) {
        console.error("[ws] failed to send queued message", e);
      }
    }
    outboundQueue = [];
  };

  const scheduleReconnect = () => {
    if (reconnectAttempts >= MAX_RECONNECT_ATTEMPTS) {
      console.warn("[ws] max reconnect attempts reached");
      return;
    }

    const delay = Math.min(1000 * Math.pow(2, reconnectAttempts), 10000);
    reconnectAttempts++;

    console.log(`[ws] reconnecting in ${delay}ms (attempt ${reconnectAttempts})`);
    setTimeout(() => api.connect(), delay);
  };

  const addHandler = <T>(set: Set<T>, h: T): Unsubscribe => {
    set.add(h);
    return () => set.delete(h);
  };

  const api: WSHandle = {
    get socket() {
      return socket;
    },

    connect() {
      if (!WS_URL || isOpen() || connecting) return;
      connecting = true;
      console.log("[ws] connecting to", WS_URL);

      try {
        const s = new WebSocket(WS_URL);

        s.onopen = () => {
          socket = s;
          connecting = false;
          reconnectAttempts = 0;
          flush(s);
          openHandlers.forEach((h) => h());
          console.log("[ws] ✅ connected");
        };

        s.onmessage = (ev) => {
          try {
            const msg = JSON.parse(ev.data) as ServerMsg;
            console.log("[ws] <= ", msg);
            messageHandlers.forEach((h) => h(msg));
          } catch (e) {
            console.warn("[ws] failed to parse message:", ev.data, e);
          }
        };

        s.onclose = (ev) => {
          if (socket === s) socket = null;
          connecting = false;
          closeHandlers.forEach((h) => h(ev.code, ev.reason));

          if (ev.code !== 1000) {
            console.log("[ws] ❌ closed unexpectedly", ev.code, ev.reason);
            scheduleReconnect();
          } else {
            console.log("[ws] closed normally");
          }
        };

        s.onerror = (ev) => {
          console.error("[ws] ❌ error", ev);
          connecting = false;
        };
      } catch (e) {
        console.error("[ws] failed to create WebSocket:", e);
        connecting = false;
        scheduleReconnect();
      }
    },

    send(msg: ClientMsg) {
      if (!isOpen()) {
        console.log("[ws] queueing message (not connected)", msg);
        outboundQueue.push(msg);
        api.connect();
        return;
      }

      try {
        socket?.send(JSON.stringify(msg));
        console.log("[ws] => ", msg);
      } catch (e) {
        console.error("[ws] failed to send message:", e);
        outboundQueue.push(msg);
      }
    },

    close() {
      reconnectAttempts = MAX_RECONNECT_ATTEMPTS;
      socket?.close(1000, "User closed connection");
      socket = null;
      connecting = false;
      outboundQueue = [];
      console.log("[ws] connection closed by user");
    },

    onMessage(handler: (msg: ServerMsg) => void) {
      return addHandler(messageHandlers, handler);
    },

    onOpen(handler: () => void) {
      return addHandler(openHandlers, handler);
    },

    onClose(handler: (code: number, reason: string) => void) {
      return addHandler(closeHandlers, handler);
    },
  };

  return api;
}

export const ws: WSHandle = (window.__TTT_WS__ ??= createWS());