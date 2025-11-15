// client/src/pages/CreateJoin.tsx
import { useState, useEffect } from "react";
import { Button, TextInput, Title, Stack, Alert, Text, Divider } from "@mantine/core";
import { ws } from "../api/ws";
import { useStore } from "../state/store";
import { useNavigate } from "react-router-dom";

export default function CreateJoin() {
  const navigate = useNavigate();
  const {
    playerName,
    setPlayerName,
    gameId,
    setGameId,
    setPlayers,
    setBoard,
    setStatus,
    setWhosTurn,
    gameType,
  } = useStore();

  const [joinCode, setJoinCode] = useState("");
  const [error, setError] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [isJoining, setIsJoining] = useState(false);

  useEffect(() => {
    ws.connect();
  }, []);

  // Navigate when matchId is set by server response
  useEffect(() => {
    if (gameId) {
      console.log(`[CreateJoin] Navigating to match ${gameId}`);
      navigate(`/match/${gameId}`);
    }
  }, [gameId, navigate]);

  const onCreate = () => {
    if (!playerName.trim()) {
      setError("Enter your name");
      return;
    }

    if (!gameType) {
      setError("Game type not selected");
      return;
    }

    setError("");
    setIsCreating(true);

    // Generate new game ID
    const newGameId = Math.random().toString(36).substring(2, 8).toUpperCase();

    // Clear any previous game state
    setPlayers([]);
    setBoard([
      [0, 0, 0],
      [0, 0, 0],
      [0, 0, 0],
    ]);
    setStatus("waiting");
    setWhosTurn("");

    // Store player name in sessionStorage
    sessionStorage.setItem("playerName", playerName);

    console.log(`[CreateJoin] Creating ${gameType} game ${newGameId} for ${playerName}`);
    setGameId(newGameId);

    ws.send({
      type: "GameRoom",
      data: {
        game: gameType,
        action: "join",
        player_name: playerName,
        game_id: newGameId,
      },
    });

    // Reset creating state after a delay
    setTimeout(() => setIsCreating(false), 1000);
  };

  const onJoin = () => {
    if (!joinCode.trim() || !playerName.trim()) {
      setError("Enter both name and game code");
      return;
    }

    if (!gameType) {
      setError("Game type not selected");
      return;
    }

    setError("");
    setIsJoining(true);

    const game_id = joinCode.toUpperCase();

    // Clear any previous game state
    setPlayers([]);
    setBoard([
      [0, 0, 0],
      [0, 0, 0],
      [0, 0, 0],
    ]);
    setStatus("waiting");
    setWhosTurn("");

    // Store player name in sessionStorage
    sessionStorage.setItem("playerName", playerName);

    console.log(`[CreateJoin] ${playerName} joining ${gameType} game ${game_id}`);
    setGameId(game_id);

    ws.send({
      type: "GameRoom",
      data: {
        game: gameType,
        action: "join",
        player_name: playerName,
        game_id,
      },
    });

    // Reset joining state after a delay
    setTimeout(() => setIsJoining(false), 1000);
  };

  const getGameTitle = () => {
    if (gameType === "tictactoe") return "Tic Tac Toe";
    if (gameType === "rockpaperscissors") return "Rock Paper Scissors";
    return "Game";
  };

  return (
    <div
      style={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
      }}
    >
      <Stack gap="md" mt="sm" style={{ minWidth: 400 }}>
        <Title order={3} ta="center">
          {getGameTitle()} - Start or Join
        </Title>

        {error && (
          <Alert color="red" withCloseButton onClose={() => setError("")}>
            {error}
          </Alert>
        )}

        <TextInput
          label="Display name"
          placeholder="Your name"
          value={playerName}
          onChange={(e) => setPlayerName(e.currentTarget.value)}
          required
          size="md"
        />

        <Stack gap="sm">
          <Text size="sm" fw={600} c="dimmed">
            Create New Game
          </Text>
          <Button
            onClick={onCreate}
            color="teal"
            loading={isCreating}
            fullWidth
            size="lg"
          >
            🎮 Create Match
          </Button>
        </Stack>

        <Divider label="OR" labelPosition="center" />

        <Stack gap="sm">
          <Text size="sm" fw={600} c="dimmed">
            Join Existing Game
          </Text>
          <TextInput
            label="Game Code"
            placeholder="e.g. ABCD12"
            value={joinCode}
            onChange={(e) => setJoinCode(e.currentTarget.value.toUpperCase())}
            required
            size="md"
          />
          <Button
            onClick={onJoin}
            color="blue"
            loading={isJoining}
            fullWidth
            size="lg"
          >
            🚀 Join Match
          </Button>
        </Stack>
      </Stack>
    </div>
  );
}