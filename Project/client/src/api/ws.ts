// WebSocket API aligned with Rust backend protocol
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
        game_id: string;
        player_name: string;
        chat_message: string;
        time: string;
      };
    }
  | {
      type: "TicTacToe";
      data: {
        game_id: string; // ✅ Added missing field
        whos_turn: string;
        choice: string;
      };
    };

export type ServerMsg =
  | { type: "Echo"; data: { message: string } }
  | { 
      type: "GameRoom"; 
      data: {
        action: string;
        game_id: string;
        player_name?: string;
        players?: string[];
      }
    }
  | { 
      type: "Chat"; 
      data: {
        player_name: string;
        chat_message: string;
        time: string;
      }
    }
  | { 
      type: "TicTacToe"; 
      data: {
        board?: number[][];
        whos_turn?: string;
        status?: string;
      }
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
          reconnectAttempts = 0; // ✅ Reset on successful connection
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
          
          // ✅ Only reconnect if it wasn't a normal closure
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
      reconnectAttempts = MAX_RECONNECT_ATTEMPTS; // ✅ Prevent reconnection
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
