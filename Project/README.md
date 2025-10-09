# Project – Multiplayer Real-Time Game and Chat App

Monorepo for our Survey of Programming Languages group project.

## Team Onboarding Guide

### Repository Structure

```text
Project/
  client/   → React + TypeScript + Mantine (frontend)
  server/   → Rust (backend)
  docs/     → protocols, architecture, notes
  scripts/  → helper scripts (optional later)
  .github/  → CI/CD workflows (later)
```

- The `.gitignore` excludes `node_modules/`, `target/`, logs, and OS/editor files.  
- Work happens mainly in `client/` and `server/`.  
- Keep `docs/` updated with any design decisions or message protocol updates.  

---

### Local Setup

#### Backend (server)

```bash
cd Project/server
cargo run
```

- Compiles and runs the Rust backend.  
- For now, prints **Hello, world!**.  
- Dependencies go in `Cargo.toml`.  

#### Frontend (client)

```bash
cd Project/client
npm install
npm run dev
```

- Starts the dev server at <http://localhost:5173>.  
- Currently shows the Vite starter page.  
- Dependencies go in `package.json`.  

---

### Suggested Roles

- **Server lead** → WebSocket endpoints, game logic, state handling (Rust).  
- **Client lead** → React UI (Tic-Tac-Toe board, chat, game status).  
- **Protocol lead** → Define and update JSON message formats (join game, moves, chat).  
- **Docs/testing lead** → Maintain `docs/` and run end-to-end tests.  

---

### Next Steps

- Make sure you can run both **server** and **client** locally.  
- Decide who will own each role.  
- Begin drafting the **message protocol** in `docs/protocol.md`.  
- Start building the **layout skeleton** (client) and **WebSocket endpoint** (server).
