// client/src/pages/CreateJoin.tsx
import { useState, useEffect } from "react";
import { Button, Group, TextInput, Title, Stack, Alert } from "@mantine/core";
import { ws } from "../api/ws";
import { useStore } from "../state/store";
import { useNavigate } from "react-router-dom";

export default function CreateJoin() {
    const navigate = useNavigate();
  const { playerName, setPlayerName, gameId, setGameId } = useStore();
  const [joinCode, setJoinCode] = useState("");
  const [error, setError] = useState("");
  const [isCreating, setIsCreating] = useState(false);

  useEffect(() => {
    ws.connect();
  }, []);

  // Navigate when matchId is set by server response
 useEffect(() => {
    if (gameId) navigate(`/match/${gameId}`);
  }, [gameId, navigate]);


 
    const onCreate = () => {
    if (!playerName.trim()) {
      setError("Enter your name");
      return;
    }
    setError("");
    setIsCreating(true);

    const newGameId = Math.random().toString(36).substring(2, 6).toUpperCase();
    setGameId(newGameId);

    ws.send({
      type: "GameRoom",
      data: {
        game: "tictactoe",
        action: "join",
        player_name: playerName,
        game_id: newGameId,
      },
    });
  };

   const onJoin = () => {
    if (!joinCode.trim() || !playerName.trim()) {
      setError("Enter both name and game code");
      return;
    }

    const game_id = joinCode.toUpperCase();
    setGameId(game_id);

    ws.send({
      type: "GameRoom",
      data: {
        game: "tictactoe",
        action: "join",
        player_name: playerName,
        game_id,
      },
    });
  };

  return (
    <div
  style={{
    display: "flex",
    justifyContent: "center", // horizontal center
    alignItems: "center", // vertical center
  }}
>
    <Stack gap="md" mt="sm">
      <Title order={3}>Start or Join a Match</Title>

      {error && <Alert color="red">{error}</Alert>}

      <TextInput
        label="Display name"
        placeholder="Your name"
        value={playerName}
        onChange={(e) => setPlayerName(e.currentTarget.value)}
        required
      />

      <Group>
        <Button 
          onClick={onCreate} 
          color="teal"
          loading={isCreating}
        >
          Create Match
        </Button>
      </Group>

      <TextInput
        label="Join by code"
        placeholder="e.g. ABCD"
        value={joinCode}
        onChange={(e) => setJoinCode(e.currentTarget.value)}
        required
      />
      <Group>
        <Button 
          onClick={onJoin}
        >
          Join Match
        </Button>
      </Group>
    </Stack>
    </div>
  );
}