# Project – Multiplayer Real-Time Game and Chat App

## Contents

- [1 Description](#11-description)
  - [1.1 General Structure](#11-general-structure)
  - [1.2 Repository Structure](#12-repository-structure)
  - [1.3 Notes for Collaborators](#13-notes-for-collaborators)
- [2 Setup](#2-setup)
  - [2.1 Install git, curl, build-essential, and tmux](#21-install-git-curl-build-essential-and-tmux)
  - [2.2 Install Node.js and npm, using nvm](#22-install-nodejs-and-npm-using-nvm)
  - [2.3 Install Rust toolchain](#23-install-rust-toolchain)
  - [2.4 Verify installations](#24-verify-installations)
  - [2.5 Clone repository](#25-clone-repository)
  - [2.6 Automated environment setup and startup](#26-automated-environment-setup-and-startup)
  - [2.7 Manual environment setup](#27-manual-environment-setup)
  - [2.8 Manual server startup](#28-manual-server-startup)

---

## 1 Description

The primary goal of this project is to build a complete, distributed client-server application that facilitates real-time user interaction through both a multiplayer game environment (e.g., Tic-Tac-Toe) and an integrated text-based chat system.

The project adheres to a clear separation of concerns using a two-component client-server architecture:

- **Backend (Rust)**: Responsible for all game logic, state handling, and exposing WebSocket endpoints to maintain real-time connections with clients.
- **Frontend (React, Typescript, Mantine)**: Renders the complete user interface (including the game board, chat window, and status displays) and manages user input.

The core system centers on:

- **Message Protocol Design**: Defining a reliable and efficient communication protocol that governs the transfer of game state and chat messages between the Rust backend and the React frontend.
- **Server-Side Logic**: Implementing robust, performant game and state management within the Rust backend.
- **Client-Side UI/UX**: Developing an intuitive and responsive user interface using React that accurately reflects the real-time game state.

### 1.1 General Structure

- **Server Side** → WebSocket endpoints, game logic, state handling (Rust).  
- **Client Side** → React UI (Tic-Tac-Toe board, chat, game status).  
- **Shared** → Message protocol design, UI/UX decisions, documentation.

### 1.2 Repository Structure

```text
Project/
  client/   → React + TypeScript + Mantine (frontend)
  server/   → Rust (backend)
  docs/     → protocols, architecture, notes
  scripts/  → helper scripts (optional later)
  .github/  → CI/CD workflows (later)
```

### 1.3 Notes for Collaborators

- See [architecture.md](docs/architecture.md) for design details.
- Make sure you can run both **server** and **client** locally.  
- Begin implementing something from the suggested work list.
- Regularly commit and push your changes to the repo.
- Update `docs/` with new useful information for the team or for presenting.
- The `.gitignore` excludes `node_modules/`, `target/`, logs, and OS/editor files.  
- Work happens mainly in `client/` and `server/`.  
- Keep `docs/` updated with any design decisions or message protocol updates. 

## 2 Setup

This setup guide assumes the use of the **Ubuntu** operating system and should be adjusted as needed depending on other system configurations. 

Prerequisites include the following:
- git
- curl
- tmux
- Node.js
- npm
- Rust toolchain

### 2.1 Install git, curl, build-essential and tmux

```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y git curl build-essential tmux
```

### 2.2 Install Node.js and npm, using nvm

```bash
# Install nvm (Node Version Manager)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash

# Load nvm (or restart your terminal)
source ~/.nvm/nvm.sh

# Install Node.js LTS
nvm install --lts
```

### 2.3 Install Rust toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2.4 Verify installations

To verify all dependencies are installed:

```bash
node -v
npm -v
rustc --version
cargo --version
```

### 2.5 Clone repository

```bash
git clone https://github.com/arlemoine/CMPS401.git
```

### 2.6 Automated environment setup and startup

Navigate to `/Project/` and run the following:

```bash
./run.sh
```

### 2.7 Manual environment setup

Navigate to `/Project/client/` and configure the frontend environment as follows:

```bash
echo "VITE_WS_URL=ws://127.0.0.1:3001/ws" > .env
npm install
npm install vite
```

### 2.8 Manual server startup

1. In one terminal environment, navigate to `/Project/server/` and run the backend server:

```bash
cargo run
```

2. In another terminal environment, navigate to `/Project/client/` and run the frontend server:

```bash
npm run dev
```

3. Navigate to http://localhost:5173 in your web browser to use the program.
