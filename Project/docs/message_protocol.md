# WebSocket Message Protocol For Frontend-Backend Communication

## Purpose

This document defines the communication contract between the frontend and backend components of the project. It specifies the structure and types of messages exchanged over WebSocket connections to ensure consistent and reliable interaction.

## Overview

All messages use a common JSON envelope:

```json
{
  "type": "MessageType",
  "data": {}
}
```

Direction is implied by context:

- Client -> Server: action or request payload from frontend.
- Server -> Client: state broadcast or error/result.

## Core Message Types

1. [Echo](#1-echo)
2. [GameRoom](#2-gameroom)
3. [Chat](#3-chat)
4. [TicTacToe](#4-tictactoe)
5. [RockPaperScissors](#5-rockpaperscissors)
6. [Uno](#6-uno)
7. [Air Hockey](#7-air-hockey)

---

### 1. Echo

Used for connectivity tests and server-issued informative or error messages (e.g. malformed JSON).

**Client -> Server (simple echo test):**

```json
{
  "type": "Echo",
  "data": { "message": "Hello, world!" }
}
```

**Server -> Client (example malformed JSON response):**

```json
{
  "type": "Echo",
  "data": {
    "message": "Invalid JSON format for ClientMessage, <error details>"
  }
}
```

---

### 2. GameRoom

Join, leave, or reset a game room. If joining a non-existent room it is created. Supported `game` values: `tictactoe`, `rockpaperscissors`, `uno`.

Actions:

- `join`
- `leave`
- `reset` (clears in-room game state; players remain)

**Client -> Server (join):**

```json
{
  "type": "GameRoom",
  "data": {
    "game": "tictactoe",
    "action": "join",
    "player_name": "Alice",
    "game_id": "room123"
  }
}
```

**Server -> Client (broadcast join):**

```json
{
  "type": "GameRoom",
  "data": {
    "game": "tictactoe",
    "action": "join",
    "player_name": "Alice",
    "game_id": "room123"
  }
}
```

**Client -> Server (leave):**

```json
{
  "type": "GameRoom",
  "data": {
    "game": "tictactoe",
    "action": "leave",
    "player_name": "Alice",
    "game_id": "room123"
  }
}
```

**Client -> Server (reset – example for RockPaperScissors):**

```json
{
  "type": "GameRoom",
  "data": {
    "game": "rockpaperscissors",
    "action": "reset",
    "player_name": "Ada",
    "game_id": "rps001"
  }
}
```

---

### 3. Chat

Client sends with `action: "send"`. Server rebroadcasts with `action: "broadcast"` and timestamp filled in. `time` is server-generated.

**Client -> Server:**

```json
{
  "type": "Chat",
  "data": {
    "action": "send",
    "game_id": "room123",
    "player_name": "Alice",
    "chat_message": "Hello Bob!",
    "time": ""
  }
}
```

**Server -> Client (broadcast):**

```json
{
  "type": "Chat",
  "data": {
    "action": "broadcast",
    "game_id": "room123",
    "player_name": "Alice",
    "chat_message": "Hello Bob!",
    "time": "03:27 PM"
  }
}
```

---

### 4. TicTacToe

Board is a 1D array of 9 strings: indices 0..8 map to:
Row A: A1=0, A2=1, A3=2
Row B: B1=3, B2=4, B3=5
Row C: C1=6, C2=7, C3=8

Cell values: "X", "O", "" (empty). Server tracks turn and status.

Client only sends attempted move with `choice` coordinate and `whos_turn` (the player's own name attempting the move).

Statuses:

- IN_PROGRESS
- invalid_move (cell occupied or bad coordinate)
- invalid_player (not that player's turn / not in room)
- gameover_x
- gameover_o
- gameover_tie

**Client -> Server (move):**

```json
{
  "type": "TicTacToe",
  "data": {
    "game_id": "room123",
    "whos_turn": "Alice",
    "choice": "A1"
  }
}
```

**Server -> Client (state broadcast after valid move):**

```json
{
  "type": "TicTacToe",
  "data": {
    "board": ["X", "", "", "", "", "", "", "", ""],
    "whos_turn": "Bob",
    "status": "IN_PROGRESS"
  }
}
```

**Server -> Client (invalid move):**

```json
{
  "type": "TicTacToe",
  "data": {
    "board": ["X", "", "", "", "", "", "", "", ""],
    "whos_turn": "Alice",
    "status": "invalid_move"
  }
}
```

**Server -> Client (win example):**

```json
{
  "type": "TicTacToe",
  "data": {
    "board": ["X", "X", "X", "", "O", "", "", "", "O"],
    "whos_turn": "Bob",
    "status": "gameover_x"
  }
}
```

**Server -> Client (tie example):**

```json
{
  "type": "TicTacToe",
  "data": {
    "board": ["X", "O", "X", "X", "O", "O", "O", "X", "X"],
    "whos_turn": "Alice",
    "status": "gameover_tie"
  }
}
```

Reset: use `GameRoom` with `action: "reset"` for the room and `game: "tictactoe"`.

---

### 5. RockPaperScissors

Round-based; players may query state without providing a choice. Choices: `rock`, `paper`, `scissors`.

Client request:

- With choice: submit/lock the player's move for current round.
- Without choice: fetch latest state.

Server broadcast includes both player names (player1/player2 ordering is server-defined), their choices (or null), status, winner, and message.

Statuses:

- waiting_for_opponent (fewer than two players)
- waiting_for_choices (two players, no moves yet)
- waiting_for_opponent_choice (one move submitted)
- round_complete (both moves; winner resolved)
- invalid_choice
- unknown_player
- room_not_found
- wrong_game_type

Winner: player name or "tie".

**Client -> Server (query without choice):**

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Ada" }
}
```

**Server -> Client (state waiting for choices):**

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "rps001",
    "player1": "Ada",
    "player2": "Alan",
    "player1_choice": null,
    "player2_choice": null,
    "status": "waiting_for_choices",
    "winner": null,
    "message": "Waiting for both players."
  }
}
```

**Client -> Server (submit choice):**

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Ada", "choice": "rock" }
}
```

**Server -> Client (after first choice):**

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "rps001",
    "player1": "Ada",
    "player2": "Alan",
    "player1_choice": "rock",
    "player2_choice": null,
    "status": "waiting_for_opponent_choice",
    "winner": null,
    "message": "Waiting for opponent choice."
  }
}
```

**Server -> Client (round complete example winner):**

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "rps001",
    "player1": "Ada",
    "player2": "Alan",
    "player1_choice": "rock",
    "player2_choice": "scissors",
    "status": "round_complete",
    "winner": "Ada",
    "message": "Ada wins this round!"
  }
}
```

**Server -> Client (tie):**

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "rps001",
    "player1": "Ada",
    "player2": "Alan",
    "player1_choice": "rock",
    "player2_choice": "rock",
    "status": "round_complete",
    "winner": "tie",
    "message": "Round is a tie."
  }
}
```

**Error (invalid choice):**

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "rps001",
    "status": "invalid_choice",
    "message": "Choice must be rock, paper, or scissors."
  }
}
```

**Error (unknown player):**

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "rps001",
    "status": "unknown_player",
    "message": "Player not in this room."
  }
}
```

**Error (room not found):**

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "nope123",
    "status": "room_not_found",
    "message": "Room not found."
  }
}
```

Reset: use `GameRoom` with `action: "reset"` and `game: "rockpaperscissors"`.

---

### 6. Uno

Uno is a turn-based multiplayer card game. The backend is authoritative: it validates all moves, enforces turn order, rejects illegal plays, applies card effects (e.g. skip, draw two), and broadcasts full public state after each action.

All Uno messages use the same envelope used by other games:

```json
{
  "type": "Uno",
  "data": { ... }
}
```

#### Client → Server Actions

- `start` — begin the game and deal cards
- `play_card` — attempt to play a card
- `draw_card` — draw exactly one card
- `pass_turn` — voluntarily end your turn
- `call_uno` — declare UNO when you have one card left
- `request_state` — fetch the most recent public game state

#### Card Format

```json
{ "color": "Red", "rank": "5" }
```

Colors: `Red`, `Yellow`, `Green`, `Blue`, `Wild`  
Ranks: `0`–`9`, `Skip`, `Reverse`, `DrawTwo`, `Wild`, `WildDrawFour`

Wild and WildDrawFour require the `choose_color` field:

```json
"choose_color": "Blue"
```

#### Example Client → Server Messages

##### Start game

```json
{
  "type": "Uno",
  "data": { "action": "start", "game_id": "room123", "player_name": "Alice" }
}
```

##### Play a card

```json
{
  "type": "Uno",
  "data": {
    "action": "play_card",
    "game_id": "room123",
    "player_name": "Alice",
    "card": { "color": "Red", "rank": "5" }
  }
}
```

##### Play Wild

```json
{
  "type": "Uno",
  "data": {
    "action": "play_card",
    "game_id": "room123",
    "player_name": "Alice",
    "card": { "color": "Wild", "rank": "Wild" },
    "choose_color": "Blue"
  }
}
```

##### Draw a card

```json
{
  "type": "Uno",
  "data": {
    "action": "draw_card",
    "game_id": "room123",
    "player_name": "Alice"
  }
}
```

##### Pass turn

```json
{
  "type": "Uno",
  "data": {
    "action": "pass_turn",
    "game_id": "room123",
    "player_name": "Alice"
  }
}
```

##### Request state

```json
{
  "type": "Uno",
  "data": {
    "action": "request_state",
    "game_id": "room123",
    "player_name": "Alice"
  }
}
```

#### Server → Client: Public Game State Broadcast

Sent after every action.

```json
{
  "type": "Uno",
  "data": {
    "game_id": "room123",
    "players": ["Ada", "Alan", "Geoffrey"],
    "current_idx": 0,
    "public_counts": [7, 7, 7],
    "top_discard": { "color": "Red", "rank": "5" },
    "chosen_color": "Red",
    "pending_draw": 0,
    "winner": null
  }
}
```

Meaning of fields:

- `players` — list of players in turn order
- `current_idx` — whose turn it is
- `public_counts` — number of cards each player holds
- `top_discard` — visible discard
- `chosen_color` — color chosen after Wild/WDF
- `pending_draw` — number of cards the next player must draw
- `winner` — name of winning player, if any

#### Server → Client: Private Hand Message

Only sent to the specific player.

```json
{
  "type": "Uno",
  "data": {
    "game_id": "room123",
    "hand": [
      { "color": "Red", "rank": "5" },
      { "color": "Green", "rank": "Reverse" }
    ]
  }
}
```

#### Game Lifecycle & Rules Summary

Uno begins when **at least two players have joined the room**. A player may send `action: "start"` at any time after two players are present. Additional players (3–10 recommended) may still join **before** the first round begins.

After the game starts:

- Cards are dealt (default: 7 per player).
- A top discard card is revealed.
- `current_idx` is set to the first player in the `players` list.
- The server immediately broadcasts the first public snapshot.
- Each player receives a **private hand** message.

General rules enforced by backend:

- Only the player whose index matches `current_idx` may act.
- Every action (play/draw/pass) results in a **new broadcast**.
- Illegal plays are rejected automatically (wrong turn, card not matchable, missing choose_color, card not owned, etc.).
- Playing a Wild or WildDrawFour **always requires** sending `choose_color`.
- After Wild/WDF, the discard color becomes the chosen color.
- Wild and WildDrawFour can be played at any time.
- DrawTwo and WildDrawFour generate a numeric `pending_draw` penalty.
- Pending draw applies at the start of the penalized player's turn.
- After pending draw resolves, the penalized player is **skipped**.
- After each turn, the next player is `(current_idx + step) % players.len()`.
- Reverse flips turn direction (only matters for 3+ players).
- Skip simply jumps over the next player.
- A player wins when their private hand length becomes 0, and `winner` is included in broadcasts.
- After a win, the room may be reset using `GameRoom: { action: "reset" }`.

---

## 7. Air Hockey

Air Hockey is a real-time, two-player game where players control paddles to hit a puck. The backend is authoritative for puck physics, collisions, and scoring. Clients send paddle positions and velocities, or optionally request the full authoritative game state.

All Air Hockey messages use the standard JSON envelope:

```json
{
  "type": "AirHockey",
  "data": { ... }
}
```

### 7.1 Client → Server Messages

#### 7.1.1 Paddle Movement

Sent continuously while a player moves their paddle. Positions and velocities are float values, and `timestamp` is a Unix timestamp in seconds with fractional sub-second precision.

```json
{
  "type": "AirHockey",
  "data": {
    "action": "move_paddle",
    "game_id": "ah001",
    "player_id": "p1",
    "position": { "x": 512.4, "y": 340.2 },
    "velocity": { "x": 3.1, "y": -0.4 }
  }
}
```

#### 7.1.2 Request Full Game State

Sent when the client wants a complete snapshot (e.g., on reconnect). Optional fields are omitted.

```json
{
  "type": "AirHockey",
  "data": {
    "action": "request_state",
    "game_id": "ah001",
    "player_id": "p1"
  }
}
```

- `position` and `velocity` are omitted because they are not needed for a full state request.

### 7.2 Server → Client Messages

The server broadcasts the full game state periodically. It includes paddle positions and velocities, puck state, score, and optional events.

```json
{
  "type": "AirHockey",
  "data": {
    "event": "update",
    "game_id": "ah001",
    "timestamp": 1731429871.25,
    "paddles": {
      "p1": { "x": 512.4, "y": 340.2, "vx": 3.1, "vy": -0.4 },
      "p2": { "x": 490.0, "y": 320.1, "vx": 0.0, "vy": 1.2 }
    },
    "puck": { "x": 640.0, "y": 360.0, "vx": 2.5, "vy": -3.1 },
    "score": { "p1": 2, "p2": 3 }
  }
}
```

- "paddles" is a map from player_id → PaddleState, where each PaddleState contains x, y, vx, vy.

#### 7.2.1 Game Events

Optional discrete events are included only when relevant:

```json
{
  "type": "AirHockey",
  "data": {
    "event": "p2_score",
    "game_id": "ah001",
    "timestamp": 1731429882.378,
    "paddles": {
      "p1": { "x": 512.4, "y": 340.2, "vx": 3.1, "vy": -0.4 },
      "p2": { "x": 490.0, "y": 320.1, "vx": 0.0, "vy": 1.2 }
    },
    "puck": { "x": 640.0, "y": 360.0, "vx": 2.5, "vy": -3.1 },
    "score": { "p1": 2, "p2": 4 }
  }
}
```

```json
{
  "type": "AirHockey",
  "data": {
    "event": "game_over_winner_p2",
    "game_id": "ah001",
    "timestamp": 1731429882.352,
    "paddles": {
      "p1": { "x": 512.4, "y": 340.2, "vx": 3.1, "vy": -0.4 },
      "p2": { "x": 490.0, "y": 320.1, "vx": 0.0, "vy": 1.2 }
    },
    "puck": { "x": 640.0, "y": 360.0, "vx": 2.5, "vy": -3.1 },
    "score": { "p1": 2, "p2": 4 }
  }
}
```

### 7.3 Notes

- Paddle movement messages are **frequent and incremental**, allowing smooth interpolation on the client.
- `timestamp` is used for latency compensation and client-side interpolation.
- Backend is authoritative for puck physics, collisions, and scoring.
- Clients should implement **local prediction** for their own paddle and **interpolation** for opponent paddles and the puck to reduce perceived lag.

---

## Adding New Message Types

1. **Define the new message type name and its data schema.**
2. **Implement backend support for handling and sending the message.**
3. **Update this protocol document to include the new message type with examples.**

Maintaining this document up-to-date is critical for smooth collaboration and integration.
