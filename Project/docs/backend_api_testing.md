# Backend API Testing with Websocat

This document describes how to manually test your WebSocket backend using the `websocat` utility in a Linux terminal.

## 1. Prerequisites

- Ensure your backend is running on the correct port (default assumed: `ws://localhost:3000/ws`).
- Install [`websocat`](https://github.com/vi/websocat):
  ```bash
  cargo install websocat

## 2. Starting the backend

```bash
cargo run
```

Make sure there are no compilation errors.

## 3. Open two WebSocket clients

Open **two separate terminal windows**.

### Terminal 1 (User A)

```bash
websocat ws://localhost:3000/ws
```

### Terminal 2 (User B)

```bash
websocat ws://localhost:3000/ws
```

## 4. Test flow

### 4.1 User A joins a game room

Send the following JSON in Terminal 1:

```json
{"type":"GameRoom","data":{"game":"tictactoe","action":"join","player_name":"Alice","game_id":"room123"}}
```

**Expected server response:**

```json
{"type":"GameRoom","data":{"game":"tictactoe","action":"join","player_name":"Alice","game_id":"room123"}}
```

### 4.2 User B joins the same game room

Send in Terminal 2:

```json
{"type":"GameRoom","data":{"game":"tictactoe","action":"join","player_name":"Bob","game_id":"room123"}}
```

**Expected behavior:**

* Both terminals receive:

```json
{"type":"GameRoom","data":{"game":"tictactoe","action":"join","player_name":"Bob","game_id":"room123"}}
```

### 4.3 User A sends a chat message

Send in Terminal 1:

```json
{"type":"Chat","data":{"action":"send","game_id":"room123","player_name":"Alice","chat_message":"Hello Bob!","time":""}}
```

**Expected:**

* Both terminals receive the broadcast:

```json
{"type":"Chat","data":{"action":"send","game_id":"room123","player_name":"Alice","chat_message":"Hello Bob!","time":"03:27 PM"}}
```

*(time will match server timestamp)*

### 4.4 User B sends a chat message

Send in Terminal 2:

```json
{"type":"Chat","data":{"action":"send","game_id":"room123","player_name":"Bob","chat_message":"Hey Alice!","time":""}}
```

**Expected:**

* Both terminals receive Bob’s chat message with server timestamp.

### 4.5 User A leaves the game room

Send in Terminal 1:

```json
{"type":"GameRoom","data":{"action":"leave","player_name":"Alice","game_id":"room123"}}
```

**Expected:**

* Alice removed from room
* Alice’s tx removed from room’s list

### 4.6 User B sends a chat after Alice left

Send in Terminal 2:

```json
{"type":"Chat","data":{"action":"send","game_id":"room123","player_name":"Bob","chat_message":"Still here!","time":""}}
```

**Expected:**

* Only Bob receives the message (Alice has left)

### 4.7 User B leaves the game room

Send in Terminal 2:

```json
{"type":"GameRoom","data":{"action":"leave","player_name":"Bob","game_id":"room123"}}
```

**Expected:**

* Room is now empty
* Room is deleted from `AppState`

### 4.8 Testing malformed JSON

Send in any terminal:

```json
{"invalid":"json"}
```

## 4.9 TicTacToe gameplay between two players

#### 4.9.1 User A makes the first move

Send in Terminal 1:

```json
{"type":"TicTacToe","data":{"whos_turn":"Alice","choice":"A1"}}
```

**Expected server response to both players:**

```json
{
  "type":"TicTacToe",
  "data":{
    "board":["X","","","","","","","",""],
    "whos_turn":"o",
    "status":"next_o"
  }
}
```

#### 4.9.2 User B makes a move

Send in Terminal 2:

```json
{"type":"TicTacToe","data":{"choice":"B2","game_id":"room123","player_name":"Bob"}}
```

**Expected server response:**

```json
{
  "type":"TicTacToe",
  "data":{
    "board":["X","","","","O","","","",""],
    "whos_turn":"x",
    "status":"next_x"
  }
}
```

#### 4.9.3 Continue gameplay until game ends

Players take turns sending moves:

* Valid moves update the board and switch `whos_turn`.
* Invalid moves (occupied cell or invalid choice) return:

```json
{
  "type":"TicTacToe",
  "data":{
    "board":[...current board...],
    "whos_turn":"x" or "o",
    "status":"invalid_move"
  }
}
```

* When a player wins:

```json
{
  "type":"TicTacToe",
  "data":{
    "board":[...final board...],
    "whos_turn":"x" or "o",
    "status":"gameover_x" or "gameover_o"
  }
}
```

* When the board is full and no winner:

```json
{
  "type":"TicTacToe",
  "data":{
    "board":[...final board...],
    "whos_turn":"x" or "o",
    "status":"gameover_tie"
  }
}
```

**Expected server response:**

```json
{"type":"Echo","data":{"message":"Invalid JSON format for ClientMessage"}}
```

## 5. Notes

* Replace `room123` with any room ID you want to test.
* Player names must be unique per room for this test to work correctly.
* Time fields in chat messages are automatically set by the backend.
* This testing setup is useful for validating the **full WebSocket flow** without a frontend.
