# Architecture Guide – Tic‑Tac‑Toe (React + Mantine ↔ Rust/Axum via WebSockets)

> **Goal (prototype):** Two players join the same room, see a shared 3×3 board, make legal moves in turn, chat in a sidebar, and the server enforces rules & declares win/draw. No database for now.

---

## 1) Big Picture

```text
┌────────────────────────┐      WebSocket JSON      ┌────────────────────────┐
│        React UI        │  <────────────────────>  │        Rust Server     │
│  (TypeScript + Mantine)│                          │   (Axum + Tokio async) │
└────────────────────────┘                          └───────────┬────────────┘
               Global UI state (Zustand)                        │
                                                                │
                                      ┌─────────────────────────┴─────────────────────┐
                                      │         Match Registry (in‑memory)            │
                                      │   HashMap<MatchId, MatchRoom/Actor>           │
                                      └───────────────┬───────────────────────────────┘
                                                      │ per‑match task (actor)
                                            ┌─────────┴─────────┐
                                            │  MatchRoom owns   │
                                            │  • GameState      │
                                            │  • Players        │
                                            │  • Broadcast tx   │
                                            └───────────────────┘
```

**Key idea:** Instead of many REST requests, we keep one **WebSocket** open. Clients send **events** (join, move, chat), and the server **pushes** updates (state_update, match_over) to all clients in the same room.

---

## 2) REST vs WebSockets (mental model)

- **REST (stateless):** Client repeatedly *requests* data (`GET /state`), server returns a response.
- **WebSockets (stateful):** Each client opens a persistent connection with server; **any side can send messages/game updates at any time**. Messages and updates are immediately pushed to all clients in the same room. Perfect for games/chat.

**Comparison:**

| REST endpoint                           | WebSocket event                                             |
|-----------------------------------------|-------------------------------------------------------------|
| `POST /matches`                         | `{ type: "create_match" }`                                  |
| `POST /matches/{id}/join`               | `{ type: "join_match", payload: { matchId } }`              |
| `POST /matches/{id}/moves`              | `{ type: "make_move", payload: { matchId,row,col } }`       |
| `GET /matches/{id}/state`               | **Server pushes** `{ type: "state_update", ... }`           |
| `GET /messages?since=<timestamp>`       | **Server pushes** `{ type: "chat", ... }`                   |

---

## 3) Components & Responsibilities

### Client (React + Mantine)

**React Pages:**

- `CreateJoin` → enter name; create or join by code
- `Match` → board component + chat sidebar + status banner

**Function:**

- **UI:** Board grid, chat panel, status banner, create/join screen.
- **State:** State managed in a global store: Zustand (see 4). If desynced, send `resync` to server.
- **WebSocket (WS) wrapper:** Opens socket, reconnects if dropped, queues messages until open, forwards server events to the store.

### Server (Rust + Axum + Tokio)

TLDR: The server has a central router, a registry of all games, and then one “mini-server” (actor) per game that manages moves and pushes updates. The actual game rules are a separate, clean module.

- **Axum Router (HTTP):** `GET /health` for sanity; `GET /ws` upgrades HTTP to a WebSocket.
- **Match Registry (Directory of active matches):** `HashMap<MatchId, MatchRoom>`.
  - Receives `create_match` and `join_match` requests.
  - Creates a new `MatchRoom` actor for each new match.
  - Forwards player connections to the right `MatchRoom`.
- **MatchRoom (actor):** Single task that owns the **authoritative** game state; receives commands; validates via game rules; broadcasts updates to room subscribers (Tokio `broadcast` channel).
- **Game Rules Module:** Pure functions & types for Tic‑Tac‑Toe (no I/O). Function examples: `Board`, `Mark (X|O)`, `GameState`, `apply_move`, `winner`, `is_draw`.

**Why actors?** One controller per room serializes updates → avoids race conditions and complex locking.

---

## 4) Client State Model (Zustand)

Zustand is a small, fast, and scalable state management library for React applications. Unlike React's built-in `useState` or `useReducer` hooks, which manage state locally within individual components, Zustand provides a centralized global store that can be accessed and updated from any component without passing around props.

Suggested slices (keys):

- `connection`: ws status, retries
- `me`: `{ id, name, mark }`
- `match`: `{ matchId, status }`
- `board`: `3×3` grid (`"X" | "O" | ""`)
- `chat`: list of `{ from, text, ts }`

---

## 5) Message Protocol

> Keep messages small and explicit. Every message has a `type` and `payload`.

### Client → Server

```json
{ "type": "join",         "payload": { "displayName": "Ada" } }
{ "type": "create_match",  "payload": {} }
{ "type": "join_match",    "payload": { "matchId": "ABCD" } }
{ "type": "make_move",     "payload": { "matchId": "ABCD", "row": 0, "col": 2 } }
{ "type": "chat",          "payload": { "matchId": "ABCD", "text": "glhf" } }
{ "type": "resync",        "payload": { "matchId": "ABCD" } }
```

### Server → Client

```json
{ "type": "match_created", "payload": { "matchId": "ABCD", "you": { "id":"p1","name":"Ada","mark":"X" } } }
{ "type": "state_update",  "payload": {
  "matchId":"ABCD",
  "board":[["X","",""],["","O",""],["","",""]],
  "nextTurn":"O",
  "players":[{"id":"p1","name":"Ada","mark":"X"},{"id":"p2","name":"Linus","mark":"O"}],
  "status":"IN_PROGRESS"
}}
{ "type": "match_over",    "payload": { "winner": "X", "winningLine": [[0,0],[1,1],[2,2]] } }
{ "type": "chat",           "payload": { "from":"Ada", "text":"gg", "ts":1738970000 } }
{ "type": "error",          "payload": { "code":"CELL_TAKEN", "message":"Cell (0,2) is occupied." } }
```

> **True game/chat state lives on the server.** Clients render a copy to use for display and can request `{ type: "resync" }` after refresh.

---

## 6) End‑to‑End Flow (one game)

```text
Player A opens client → ws connect → send {join}
Player A → {create_match}      → server creates room R, replies {match_created, matchId:R}
Player B opens client → {join} → {join_match, room R} → server adds Player B, broadcasts {state_update}
Player A clicks a cell         → {make_move} → server validates & updates → broadcast {state_update}
… repeat …
Server detects win/draw → broadcast {match_over}
Player B refreshes             → {resync} → server replies with latest {state_update}
```

---

## 7) Concurrency, Safety, and Errors

- **Single writer per room:** The MatchRoom actor is the only place that mutates game state → no races.
- **Validation:** Server rejects illegal moves (occupied cell, wrong turn, out‑of‑bounds) and sends `{type:"error"}`.
- **Room lifecycle:** A room exists when created; it can be GC’d after both players disconnect and a timeout elapses (optional for prototype).
- **Identity:** Anonymous by default (display name + server‑assigned `X`/`O`).

---

## 8) Environments & Config

- **Dev:**

  - Server: `ws://localhost:3001/ws` is the websocket endpoint that the React app connects to and `GET http://localhost:3001/health` is the health check endpoint.
  - Client: `http://localhost:5173` runs the React app, configure `VITE_WS_URL=ws://localhost:3001/ws` to connect to the backend.

## 9) Folder Structures (target)

### Server

```text
server/
  src/
    main.rs            # Axum router: /health, /ws
    ws.rs              # WebSocket upgrade & per‑connection loop
    types.rs           # serde enums for ClientMsg/ServerMsg
    matchmaker.rs      # MatchRegistry + MatchRoom actor (broadcast channel)
    game/
      mod.rs           # pure rules: Board, Mark, GameState, apply_move, winner
```

### Client

```text
client/
  src/
    main.tsx           # MantineProvider + Notifications
    App.tsx            # AppShell; routes CreateJoin ↔ Match
    api/ws.ts          # connect/send/reconnect/ping wrapper
    state/store.ts     # Zustand store (board, chat, status, me, match)
    views/
      CreateJoin.tsx   # join/create UI
      Match.tsx        # board + chat layout
    components/
      Board.tsx        # 3×3 grid
      Chat.tsx         # messages + input
      Status.tsx       # banners, errors
```

---

## 10) Non‑Goals (for prototype)

- Auth, persistent accounts, rankings/leaderboards
- Databases; cross‑match analytics
- UI perfection; mobile support
- Bots, AI opponents, spectators

Focus is on correctness, concurrency safety, and a smooth demo.

---
