# Project – Multiplayer Real-Time Game and Chat App

Monorepo for our Survey of Programming Languages group project.

## Repository Structure

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

## Local Setup

### Backend (server)

```bash
cd Project/server
cargo run
```

- Compiles and runs the Rust backend.  
- For now, prints **Hello, world!**.  
- Dependencies go in `Cargo.toml`.  

### Frontend (client)

```bash
cd Project/client
npm install
npm run dev
```

- Starts the dev server at <http://localhost:5173>.  
- Currently shows a Mantine starter page.  
- Dependencies go in `package.json`.  

---

### Suggested Work

- **Server Side** → WebSocket endpoints, game logic, state handling (Rust).  
- **Client Side** → React UI (Tic-Tac-Toe board, chat, game status).  
- **Shared** → Message protocol design, UI/UX decisions, documentation.

---

### Next Steps

- Make sure you can run both **server** and **client** locally.  
- Begin implementing something from the suggested work list.
- Regularly commit and push your changes to the repo.
- Update `docs/` with new useful information for the team or for presenting.
