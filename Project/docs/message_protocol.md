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

Handles operations related to game room management.

**Example:**

```json
{
  "type": "GameRoom",
  "data": {
    "roomId": "HQCU",
    "action": "join",
    "player": "player1"
  }
}
```

## Workflow for Adding New Message Types

1. **Define the new message type name and its data schema.**  
2. **Implement backend support for handling and sending the message.**  
3. **Update this protocol document to include the new message type with examples.**  

Maintaining this document up-to-date is critical for smooth collaboration and integration.
