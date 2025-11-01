// WebSocket API for Tic-Tac-Toe
export type ClientMsg =
  | { type: "join"; payload: { displayName: string } }
  | { type: "create_match"; payload: {} }
  | { type: "join_match"; payload: { matchId: string } }
  | { type: "make_move"; payload: { matchId: string; index: number } };

export type Player = { id: string; displayName: string; mark: string };

export type ServerMsg =
  | { type: "hello"; payload: { serverVersion: string } }
  | { type: "match_created"; payload: { matchId: string; you: Player } }
  | { type: "joined_match"; payload: { matchId: string; you: Player } }
  | {
      type: "state_update";
      payload: {
        matchId: string;
        players: Player[];
        status: string;
        board: (string | null)[];
        turn: string | null;
      };
    }
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

declare global {
  interface Window {
    __TTT_WS__?: ReturnType<typeof createWS>;
  }
}

function createWS(): WSHandle {
  let socket: WebSocket | null = null;
  let connecting = false;
  let outboundQueue: ClientMsg[] = [];
  
  const messageHandlers = new Set<(msg: ServerMsg) => void>();
  const openHandlers = new Set<() => void>();
  const closeHandlers = new Set<(code: number, reason: string) => void>();

  const isOpen = () => socket?.readyState === WebSocket.OPEN;

  const flush = (s: WebSocket) => {
    for (const m of outboundQueue) s.send(JSON.stringify(m));
    outboundQueue = [];
  };

  const scheduleReconnect = () => setTimeout(() => api.connect(), 500);

  const addHandler = <T, >(set: Set<T>, h: T): Unsubscribe => {
    set.add(h);
    return () => set.delete(h);
  };

  const api: WSHandle = {
    get socket() { return socket; },

    connect() {
      if (!WS_URL || isOpen() || connecting) return;
      connecting = true;
      const s = new WebSocket(WS_URL);

      s.onopen = () => {
        socket = s;
        connecting = false;
        flush(s);
        openHandlers.forEach(h => h());
        console.log("[ws] connected");
      };

      s.onmessage = (ev) => {
        try {
          const msg = JSON.parse(ev.data) as ServerMsg;
          messageHandlers.forEach(h => h(msg));
          console.log("[ws] <= ", msg);
        } catch (e) {
          console.warn("[ws] non-JSON", ev.data, e);
        }
      };

      s.onclose = (ev) => {
        if (socket === s) socket = null;
        connecting = false;
        closeHandlers.forEach(h => h(ev.code, ev.reason));
        console.log("[ws] closed", ev.code, ev.reason);
        scheduleReconnect();
      };

      s.onerror = (ev) => console.warn("[ws] error", ev);
    },

    send(msg: ClientMsg) {
      if (!isOpen()) {
        outboundQueue.push(msg);
        api.connect();
        return;
      }
      socket?.send(JSON.stringify(msg));
      console.log("[ws] => ", msg);
    },

    close() {
      socket?.close();
      socket = null;
      connecting = false;
      outboundQueue = [];
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
