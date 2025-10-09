// Project/client/src/api/ws.ts
// StrictMode + HMR-safe WebSocket singleton with queue & optional reconnect.

export type ClientMsg =
  | { type: "join"; payload: { displayName: string } }
  | { type: "create_match"; payload: {} };

export type ServerMsg =
  | { type: "hello"; payload: { serverVersion: string } }
  | { type: "match_created"; payload: { matchId: string } }
  | { type: "error"; payload: { code: string; message: string } };

const WS_URL = import.meta.env.VITE_WS_URL as string;

export type WSHandle = {
  socket: WebSocket | null;
  connect: () => void;
  send: (msg: ClientMsg) => void;
  close: () => void;
  onMessage: (handler: (msg: ServerMsg) => void) => void;
};

declare global {
  interface Window {
    __TTT_WS__?: ReturnType<typeof createWS>;
  }
}

function createWS(): WSHandle {
  let socket: WebSocket | null = null;
  let onMessageHandler: ((msg: ServerMsg) => void) | null = null;
  let outboundQueue: ClientMsg[] = [];
  let connecting = false;
  let reconnectTimer: number | null = null;
  let retryMs = 0;

  const flush = (s: WebSocket) => {
    for (const m of outboundQueue) s.send(JSON.stringify(m));
    outboundQueue = [];
  };

  const scheduleReconnect = () => {
    // backoff: 0ms, 250ms, 500ms, 1000ms (cap)
    retryMs = Math.min(retryMs ? retryMs * 2 : 250, 1000);
    if (reconnectTimer) return;
    reconnectTimer = window.setTimeout(() => {
      reconnectTimer = null;
      api.connect();
    }, retryMs);
  };

  const api: WSHandle = {
    get socket() {
      return socket;
    },
    set socket(_: WebSocket | null) {
      // no external sets
    },

    connect() {
      if (!WS_URL || !/^wss?:\/\//.test(WS_URL)) {
        console.error("[ws] VITE_WS_URL missing/invalid:", WS_URL);
        return;
      }
      if (socket && (socket.readyState === WebSocket.OPEN || socket.readyState === WebSocket.CONNECTING)) {
        return; // already open or in-flight
      }
      if (connecting) return;
      connecting = true;

      const s = new WebSocket(WS_URL);

      s.onopen = () => {
        console.log("[ws] open:", WS_URL);
        socket = s;
        connecting = false;
        retryMs = 0; // reset backoff
        flush(s);
      };

      s.onmessage = (ev) => {
        try {
          const data = JSON.parse(ev.data) as ServerMsg;
          console.log("[ws] <=", data);
          onMessageHandler?.(data);
        } catch (e) {
          console.warn("[ws] non-JSON:", ev.data, e);
        }
      };

      s.onclose = (ev) => {
        console.log("[ws] closed", ev.code, ev.reason);
        if (socket === s) socket = null;
        connecting = false;
        // reconnect in dev; avoid flapping on hard errors
        if (ev.code === 1006 || ev.code === 1001 || ev.code === 1000) {
          scheduleReconnect();
        }
      };

      s.onerror = (ev) => {
        // Don’t throw scary errors—WS will close and schedule reconnect
        console.warn("[ws] error", ev);
      };
    },

    send(msg: ClientMsg) {
      const s = socket;
      if (!s || s.readyState !== WebSocket.OPEN) {
        outboundQueue.push(msg);
        // Try to connect if not already
        if (!connecting) this.connect();
        return;
      }
      console.log("[ws] =>", msg);
      s.send(JSON.stringify(msg));
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

    onMessage(handler: (msg: ServerMsg) => void) {
      onMessageHandler = handler;
    },
  };

  return api;
}

// Reuse existing instance across HMR/StrictMode, or create once
export const ws: WSHandle = (window.__TTT_WS__ ??= createWS());