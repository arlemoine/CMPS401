# WebSocket Message Protocol for Tic-Tac-Toe Project

## Purpose

This document defines the communication contract between the frontend and backend components of the project. It specifies the structure and types of messages exchanged over WebSocket connections to ensure consistent and reliable interaction.

## Overview

All messages exchanged between the frontend and backend follow a standard JSON structure with two main fields:

- `type`: A string indicating the message type.
- `data`: An object containing the payload relevant to the message type.

```json
{
  "type": "MessageType",
  "data": { /* message-specific content */ }
}
```

This uniform structure simplifies message parsing and handling on both ends.

## Current Message Types

### 1. Echo

Used primarily for testing connectivity and latency.

**Example:**

```json
{
  "type": "Echo",
  "data": {
    "message": "Hello, world!"
  }
}
```

### 2. GameRoom

Handles operations related to joining or leaving a game room. To join a room, use the keyword "join." **If the room does not exist**, it will be created. To leave a room, use the keyword "leave." If a game has been played and it is desirable to reset the state of the game in order to play again, the keyword "reset" can be passed to the server, telling the server to reset the state of the game and pass it back to the frontend.

**Example:**

```json
{
  "type": "GameRoom",
  "data": {
    "game":"tictactoe",
    "action": "join", // "join", "leave", "reset", note that "reset" is not yet implemented
    "player_name": "John",
    "game_id": "HQCU", 
  }
}
```

### 3. Chat

Handles operations related to sending and receiving chat messages in a given game room. The frontend utilizes the "send" keyword for the action while the backend broadcasts to everyone in the game room via the "broadcast" keyword.

**Example (Client -> Server):** 

```json
{
  "type": "Chat",
  "data": {
    "game_id": "HQCU", // Specifies game room related to the chat
    "player_name": "John", // Specifies who is sending the message
    "chat_message": "Good game!", // Contents of the chat message
    "time": "" // This is left blank for a client-to-server message and then timestamped on the server side
  }
}
```

**Example (Server -> Client):** 

```json
{
  "type": "Chat",
  "data": {
    "game_id": "HQCU", // Specifies game room related to the chat
    "player_name": "John", // Specifies who message originated from
    "chat_message": "Good game!", // Contents of the chat message
    "time": "11:57 AM" // This is generated automatically via timestamp on server and sent back to frontend during broadcast to all in game room
  }
}
```

### 4. TicTacToe

Handles operations related to the game state and actions of the game TicTacToe. Note that messages differ in that only the choice made is needed to be sent to the server while the server needs to send the entire state of the game back to the frontend. The state of the game can efficiently be summed up in a handful of status messages (as well as the state of the board) as shown in the example.

**Example (Client -> Server):** 

```json
{
  "type": "TicTacToe",
  "data": {
    "whos_turn": "John", /// player_name of person who is attempting to make a move
    "choice": "A1", // Convention is A1 to C3 where letter = row and number = column
  }
}
```

**Example (Server -> Client):** 

```json
{
  "type": "TicTacToe",
  "data": {
    "board": "[[0,-1,-1],[1,0,0],[0,1,0]]", // 2D status of board with x being represented by 1's and o being represented by -1's
    "whos_turn": "John", // player_name of person whose turn it currently is
    "status": "A1", // "next_x", "next_o", "gameover_tie", "gameover_x", "gameover_o", "invalid_move", "invalid_player"
  }
}
```

## Workflow for Adding New Message Types

1. **Define the new message type name and its data schema.**  
2. **Implement backend support for handling and sending the message.**  
3. **Update this protocol document to include the new message type with examples.**  

Maintaining this document up-to-date is critical for smooth collaboration and integration.
