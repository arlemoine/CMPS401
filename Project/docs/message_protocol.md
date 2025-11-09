# WebSocket Message Protocol for Tic-Tac-Toe & Rock-Paper-Scissors

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

1. Echo
2. GameRoom
3. Chat
4. TicTacToe
5. RockPaperScissors

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

Join, leave, or reset a game room. If joining a non-existent room it is created. Supported `game` values: `tictactoe`, `rockpaperscissors`.

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

## Adding New Message Types

1. **Define the new message type name and its data schema.**
2. **Implement backend support for handling and sending the message.**
3. **Update this protocol document to include the new message type with examples.**

Maintaining this document up-to-date is critical for smooth collaboration and integration.
