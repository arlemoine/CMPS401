# Backend API Testing with Websocat

This document describes how to manually test your WebSocket backend using the `websocat` utility in a Linux terminal.

## 1. Prerequisites

- Ensure your backend is running on the correct port (default assumed: `ws://localhost:3001/ws`).
- Install [`websocat`](https://github.com/vi/websocat):

  ```bash
  cargo install websocat
  ```

## 2. Starting the backend

```bash
cargo run
```

Make sure there are no compilation errors.

## 3. Open two WebSocket clients

Open **two separate terminal windows**.

### Terminal 1 (User A)

```bash
websocat ws://localhost:3001/ws
```

### Terminal 2 (User B)

```bash
websocat ws://localhost:3001/ws
```

## 4. Test flow

### 4.1 User A joins a game room

Send the following JSON in Terminal 1:

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

**Expected server response:**

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

### 4.2 User B joins the same game room

Send in Terminal 2:

```json
{
  "type": "GameRoom",
  "data": {
    "game": "tictactoe",
    "action": "join",
    "player_name": "Bob",
    "game_id": "room123"
  }
}
```

**Expected behavior:**

- Both terminals receive:

```json
{
  "type": "GameRoom",
  "data": {
    "game": "tictactoe",
    "action": "join",
    "player_name": "Bob",
    "game_id": "room123"
  }
}
```

### 4.3 User A sends a chat message

Send in Terminal 1:

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

**Expected:**

- Both terminals receive the broadcast:

```json
{
  "type": "Chat",
  "data": {
    "game_id": "room123",
    "player_name": "Alice",
    "chat_message": "Hello Bob!",
    "time": "03:27 PM"
  }
}
```

_(time will match server timestamp)_

### 4.4 User B sends a chat message

Send in Terminal 2:

```json
{
  "type": "Chat",
  "data": {
    "game_id": "room123",
    "player_name": "Bob",
    "chat_message": "Hey Alice!",
    "time": ""
  }
}
```

**Expected:**

- Both terminals receive Bob’s chat message with server timestamp.

### 4.5 User A leaves the game room

Send in Terminal 1:

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

**Expected:**

- Alice removed from room
- Alice’s tx removed from room’s list

### 4.6 User B sends a chat after Alice left

Send in Terminal 2:

```json
{
  "type": "Chat",
  "data": {
    "game_id": "room123",
    "player_name": "Bob",
    "chat_message": "Still here!",
    "time": ""
  }
}
```

**Expected:**

- Only Bob receives the message (Alice has left)

### 4.7 User B leaves the game room

Send in Terminal 2:

```json
{
  "type": "GameRoom",
  "data": {
    "game": "tictactoe",
    "action": "leave",
    "player_name": "Bob",
    "game_id": "room123"
  }
}
```

**Expected:**

- Room is now empty
- Room is deleted from `AppState`

### 4.8 Testing malformed JSON

Send in any terminal:

```json
{ "invalid": "json" }
```

**Expected server response:**

```json
{
  "type": "Echo",
  "data": {
    "message": "Invalid JSON format for ClientMessage, <error details>"
  }
}
```

## 4.9 TicTacToe gameplay between two players

#### 4.9.1 User A makes the first move

Send in Terminal 1:

```json
{
  "type": "TicTacToe",
  "data": { "game_id": "room123", "whos_turn": "Alice", "choice": "A1" }
}
```

**Expected server response to both players:**

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

#### 4.9.2 User B makes a move

Send in Terminal 2:

```json
{
  "type": "TicTacToe",
  "data": { "game_id": "room123", "whos_turn": "Bob", "choice": "B2" }
}
```

**Expected server response:**

```json
{
  "type": "TicTacToe",
  "data": {
    "board": ["X", "", "", "", "O", "", "", "", ""],
    "whos_turn": "Alice",
    "status": "IN_PROGRESS"
  }
}
```

#### 4.9.3 Continue gameplay until game ends

Players take turns sending moves:

- Valid moves update the board and switch `whos_turn`.
- Invalid moves (occupied cell or invalid choice) return:

```json
{"type":"TicTacToe","data":{"board":[...current board...],"whos_turn":"Alice or Bob","status":"invalid_move"}}
```

- When a player wins:

```json
{"type":"TicTacToe","data":{"board":[...final board...],"whos_turn":"Alice or Bob","status":"gameover_x or gameover_o"}}
```

- When the board is full and no winner:

```json
{"type":"TicTacToe","data":{"board":[...final board...],"whos_turn":"Alice or Bob","status":"gameover_tie"}}
```

## 4.10 RockPaperScissors gameplay and messaging

These tests validate the RockPaperScissors message flow described in `message_protocol.md`.

### 4.10.1 Join a RockPaperScissors room

Terminal 1 (Player A – Alice):

```json
{
  "type": "GameRoom",
  "data": {
    "game": "rockpaperscissors",
    "action": "join",
    "player_name": "Ada",
    "game_id": "rps001"
  }
}
```

Terminal 2 (Player B – Bob):

```json
{
  "type": "GameRoom",
  "data": {
    "game": "rockpaperscissors",
    "action": "join",
    "player_name": "Alan",
    "game_id": "rps001"
  }
}
```

### 4.10.2 Player A queries current round (no choice yet)

Terminal 1:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Ada" }
}
```

Expected status (one of):

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "rps001",
    "player1": "Ada",
    "player2": "Bob",
    "player1_choice": null,
    "player2_choice": null,
    "status": "waiting_for_choices",
    "winner": null,
    "message": "Waiting for both players."
  }
}
```

(If Bob not joined yet you may see `waiting_for_opponent`.)

### 4.10.3 Player A submits a choice

Terminal 1:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Ada", "choice": "rock" }
}
```

Expected broadcast (example sequence):

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Ada", "choice": "rock" }
}
```

```json
{
  "type": "RockPaperScissors",
  "data": {
    "game_id": "rps001",
    "player1": "Ada",
    "player2": "Bob",
    "player1_choice": "rock",
    "player2_choice": null,
    "status": "waiting_for_opponent_choice",
    "winner": null,
    "message": "Waiting for opponent choice."
  }
}
```

### 4.10.4 Player B submits a choice (round resolves)

Terminal 2:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Alan", "choice": "scissors" }
}
```

Expected broadcast (example where Alice wins):

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

### 4.10.5 Tie scenario

Terminal 1:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Ada", "choice": "rock" }
}
```

Terminal 2:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Alan", "choice": "rock" }
}
```

Expected:

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

### 4.10.6 Start a new round

Terminal 2:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Alan", "choice": "paper" }
}
```

### 4.10.7 Refresh latest state (no new choice)

Either terminal:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Ada" }
}
```

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Alan" }
}
```

### 4.10.8 Invalid choice

Terminal 1:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Ada", "choice": "lizard" }
}
```

Expected:

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

### 4.10.9 Unknown player

Terminal 1:

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "rps001", "player_name": "Charlie", "choice": "rock" }
}
```

Expected:

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

### 4.10.10 Room not found

```json
{
  "type": "RockPaperScissors",
  "data": { "game_id": "nope123", "player_name": "Ada", "choice": "rock" }
}
```

Expected:

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

### 4.10.11 Reset game state (optional)

Terminal 1:

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

### 4.10.12 Players leave

Terminal 1:

```json
{
  "type": "GameRoom",
  "data": {
    "game": "rockpaperscissors",
    "action": "leave",
    "player_name": "Ada",
    "game_id": "rps001"
  }
}
```

Terminal 2:

```json
{
  "type": "GameRoom",
  "data": {
    "game": "rockpaperscissors",
    "action": "leave",
    "player_name": "Alan",
    "game_id": "rps001"
  }
}
```

## 5. Notes

- Replace `room123` with any room ID you want to test.
- Player names must be unique per room.
- Time fields in chat messages are automatically set by the backend.
- All JSON above is minified for direct paste into `websocat`.
