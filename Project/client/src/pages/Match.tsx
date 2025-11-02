// client/src/pages/Match.tsx
import { useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { Stack, Title, Text, Badge, Group, Button } from "@mantine/core";
import { useStore } from "../state/store";
import { ws } from "../api/ws";

export default function Match() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const {
    matchId,
    setMatchId,
    players,
    setPlayers,
    matchStatus,
    setMatchStatus,
    me,
    setMe,
    setBoard,
    setTurn
  } = useStore();

  // Sync matchId from URL
  useEffect(() => {
    if (id && id !== matchId) {
      setMatchId(id);
    }
  }, [id, matchId, setMatchId]);

  // Listen for state updates from WebSocket
  useEffect(() => {
    if (!id) return;
    
    ws.connect();

    const unsub = ws.onMessage((msg) => {
      console.log("[Match] Received message:", msg);
      
      if (msg.type === "state_update" && msg.payload.matchId === id) {
        console.log("[Match] state_update received:", msg.payload);
        const { players, status, board, turn } = msg.payload;
        setPlayers(players);
        setMatchStatus(status);
        setBoard(board || Array(9).fill(null));
        setTurn(turn);
        
        // Auto-navigate to board when game starts
        if (status === "IN_PROGRESS") {
          navigate(`/board/${id}`);
        }
      }

      if (msg.type === "joined_match") {
        setMe(msg.payload.you);
      }

      if (msg.type === "match_created") {
        setMe(msg.payload.you);
      }

      if (msg.type === "error") {
        console.warn("[Match] error:", msg.payload);
        alert(msg.payload.message);
      }
    });

    return () => unsub();
  }, [id, setPlayers, setMatchStatus, setMe, setBoard, setTurn, navigate]);

  const handleGoToBoard = () => {
    if (id && matchStatus === "IN_PROGRESS") {
      navigate(`/board/${id}`);
    }
  };

  const handlePlayAgain = () => {
    if (!id) return;
    // Reset local board state
  
    navigate("/createjoin");
  };

  const handleMainMenu = () => {
    navigate("/");
  };

  return (
    <Stack gap="md" mt="lg">
      <Title order={3}>Match: {matchId ?? id}</Title>

      <Group>
        <Text>Status:</Text>
        <Badge
          color={
            matchStatus === "IN_PROGRESS"
              ? "green"
              : matchStatus === "FINISHED"
              ? "red"
              : "yellow"
          }
        >
          {matchStatus ?? "WAITING"}
        </Badge>
      </Group>

      {me && (
        <Text>
          You are: <strong>{me.displayName}</strong> playing as{" "}
          <strong>{me.mark}</strong>
        </Text>
      )}

      <Stack gap="xs">
        <Text fw={600}>Players ({players.length}/2):</Text>
        {players.map((p) => (
          <Group key={p.id}>
            <Badge color={p.mark === "X" ? "blue" : "red"}>{p.mark}</Badge>
            <Text>{p.displayName}</Text>
            {p.id === me?.id && <Text c="dimmed">(You)</Text>}
          </Group>
        ))}
        {players.length < 2 && (
          <Text c="dimmed">Waiting for another player to join...</Text>
        )}
      </Stack>

      {matchStatus === "IN_PROGRESS" && players.length === 2 && (
        <Stack gap="sm">
          <Text c="green">Game is ready! Both players have joined.</Text>
          <Button onClick={handleGoToBoard} color="blue">
            Go to Game Board
          </Button>
        </Stack>
      )}
      
      {matchStatus === "WAITING" && players.length === 2 && (
        <Text c="orange">Waiting for game to start...</Text>
      )}

       <Group mt="lg" justify="center">
        <Button variant="outline" color="Green" onClick={handleMainMenu}>
          🏠 Main Menu
        </Button>
        <Button variant="filled" color="teal" onClick={handlePlayAgain}>
          🔁 Play Again
        </Button>
      </Group>
    </Stack>
  );
}