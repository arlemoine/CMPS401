// Project/client/src/api/ws.ts
// StrictMode + HMR-safe WebSocket singleton with queue, minimal reconnect,
// and a small pub/sub API to avoid repetitive single handlers.

export type ClientMsg =
  | { type: "join"; payload: { displayName: string } }
  | { type: "create_match"; payload: {} };

export type ServerMsg =
  | { type: "hello"; payload: { serverVersion: string } }
  | { type: "match_created"; payload: { matchId: string } }
  | { type: "error"; payload: { code: string; message: string } };

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

// --- HMR/StrictMode-proof singleton on window ---
declare global {
  interface Window {
    __TTT_WS__?: ReturnType<typeof createWS>;
  }
}

function createWS(): WSHandle {
  let socket: WebSocket | null = null;
  let connecting = false;
  let reconnectTimer: number | null = null;
  let retryMs = 0;
  let outboundQueue: ClientMsg[] = [];

  // Pub/Sub sets (multiple listeners; unsub returns a disposer)
  const messageHandlers = new Set<(msg: ServerMsg) => void>();
  const openHandlers = new Set<() => void>();
  const closeHandlers = new Set<(code: number, reason: string) => void>();

  const isOpen = () => socket?.readyState === WebSocket.OPEN;
  const isConnecting = () => socket?.readyState === WebSocket.CONNECTING || connecting;

  const flush = (s: WebSocket) => {
    if (outboundQueue.length === 0) return;
    for (const m of outboundQueue) s.send(JSON.stringify(m));
    outboundQueue = [];
  };

  const scheduleReconnect = () => {
    // backoff: 250ms, 500ms, 1000ms (cap)
    retryMs = Math.min(retryMs ? retryMs * 2 : 250, 1000);
    if (reconnectTimer) return;
    reconnectTimer = window.setTimeout(() => {
      reconnectTimer = null;
      api.connect();
    }, retryMs);
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
      if (!WS_URL || !/^wss?:\/\//.test(WS_URL)) {
        console.error("[ws] VITE_WS_URL missing/invalid:", WS_URL);
        return;
      }
      if (isOpen() || isConnecting()) return; // already open or in-flight

      connecting = true;
      const s = new WebSocket(WS_URL);

      s.onopen = () => {
        socket = s;
        connecting = false;
        retryMs = 0; // reset backoff
        // Notify and flush after we are officially open
        openHandlers.forEach((h) => h());
        flush(s);
        console.log("[ws] open:", WS_URL);
      };

      s.onmessage = (ev) => {
        try {
          const data = JSON.parse(ev.data) as ServerMsg;
          messageHandlers.forEach((h) => h(data));
          console.log("[ws] <=", data);
        } catch (e) {
          console.warn("[ws] non-JSON:", ev.data, e);
        }
      };

      s.onclose = (ev) => {
        if (socket === s) socket = null;
        connecting = false;
        closeHandlers.forEach((h) => h(ev.code, ev.reason));
        console.log("[ws] closed", ev.code, ev.reason);
        // reconnect in dev for common transient codes
        if (ev.code === 1006 || ev.code === 1001 || ev.code === 1000) {
          scheduleReconnect();
        }
      };

      s.onerror = (ev) => {
        // The socket will shortly close and trigger onclose; keep logs minimal
        console.warn("[ws] error", ev);
      };
    },

    send(msg: ClientMsg) {
      const s = socket;
      if (!s || !isOpen()) {
        outboundQueue.push(msg);
        if (!isConnecting()) this.connect();
        return;
      }
      s.send(JSON.stringify(msg));
      console.log("[ws] =>", msg);
    },

    close() {
      if (reconnectTimer) {
        window.clearTimeout(reconnectTimer);
        reconnectTimer = null;
      }
      socket?.close();
      socket = null;
      connecting = false;
    },

    onMessage(handler: (msg: ServerMsg) => void): Unsubscribe {
      return addHandler(messageHandlers, handler);
    },

    onOpen(handler: () => void): Unsubscribe {
      return addHandler(openHandlers, handler);
    },

    onClose(handler: (code: number, reason: string) => void): Unsubscribe {
      return addHandler(closeHandlers, handler);
    },
  };

  return api;
}

// Reuse existing instance across HMR/StrictMode, or create once
export const ws: WSHandle = (window.__TTT_WS__ ??= createWS());