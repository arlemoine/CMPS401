// client/src/pages/Match.tsx
import { useEffect } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { Stack, Title, Text, Badge, Group, Button } from "@mantine/core";
import { useStore } from "../state/store";
import { ws } from "../api/ws";
import ChatBox from "../components/ChatBox";

export default function Match() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  const {
    playerName,
    gameId,
    setGameId,
    players,
    addPlayer,
    setBoard,
    setWhosTurn,
    status,
    setStatus,
    addChatMessage,
  } = useStore();

  // Sync gameId from URL
  useEffect(() => {
    if (id && id !== gameId) setGameId(id);
  }, [id, gameId, setGameId]);

  useEffect(() => {
    ws.connect();

    const unsub = ws.onMessage((msg) => {
      console.log("[Match] Received:", msg);

      // --- Handle TicTacToe updates ---
      if (msg.type === "TicTacToe") {
        const { board, whos_turn, status: gameStatus } = msg.data;
        if (board) setBoard(board);
        if (whos_turn) setWhosTurn(whos_turn);
        if (gameStatus) setStatus(gameStatus);

        // Navigate to board once game is active
        if (gameStatus === "IN_PROGRESS") {
          navigate(`/board/${id}`);
        }
      }

      // --- Handle GameRoom updates ---
      if (msg.type === "GameRoom") {
        const { action, game_id, player_name } = msg.data;

        if (action === "join") {
          setGameId(game_id);

          const currentPlayers = useStore.getState().players;
          if (!currentPlayers.includes(player_name)) {
            addPlayer(player_name);
          }

          // Start game only when exactly 2 unique players joined
          const updatedPlayers = useStore.getState().players;
          if (updatedPlayers.length === 2 && status === "waiting") {
            setStatus("next_X"); // or next_O depending on your game rules
            navigate(`/match/${game_id}`);
          }
        }

        if (action === "reset") {
          setBoard([
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
          ]);
          setStatus("waiting");
        }
      }

      // --- Handle Chat messages ---
      if (msg.type === "Chat") {
        addChatMessage(msg.data);
      }
    });

    // Auto-join game room
    if (id && playerName) {
      ws.send({
        type: "GameRoom",
        data: {
          game: "tictactoe", // lowercase to match backend
          action: "join",
          player_name: playerName,
          game_id: id,
        },
      });
    }

    return () => unsub();
  }, [
    id,
    playerName,
    setBoard,
    setWhosTurn,
    setStatus,
    setGameId,
    addPlayer,
    addChatMessage,
    navigate,
  ]);

  const handleGoToBoard = () => navigate(`/board/${id}`);

  const handlePlayAgain = () => {
    if (!id) return;
    ws.send({
      type: "GameRoom",
      data: {
        game: "tictactoe",
        action: "reset",
        player_name: playerName,
        game_id: id,
      },
    });
    setStatus("waiting");
  };

  const handleMainMenu = () => {
    ws.send({
      type: "GameRoom",
      data: {
        game: "tictactoe",
        action: "leave",
        player_name: playerName,
        game_id: id ?? "",
      },
    });
    navigate("/");
  };

  const matchStatus = status.startsWith("gameover")
    ? "FINISHED"
    : status === "waiting"
    ? "WAITING"
    : "IN_PROGRESS";

  return (
    <Stack gap="md" mt="lg">
      <Title order={3}>Game Room: {id}</Title>

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
          {matchStatus}
        </Badge>
      </Group>

      <Text>
        You are: <strong>{playerName || "Anonymous"}</strong>
      </Text>

      <Stack gap="xs">
        <Text fw={600}>Players ({players.length}/2):</Text>
        {players.map((p, i) => (
          <Text key={i}>
            {p} {p === playerName && "(You)"}
          </Text>
        ))}
        {players.length < 2 && (
          <Text c="dimmed">Waiting for another player...</Text>
        )}
      </Stack>

      <Group mt="lg" justify="center">
        <Button variant="filled" color="blue" onClick={handleGoToBoard}>
          🎮 Go to Game Board
        </Button>
        <Button variant="filled" color="teal" onClick={handlePlayAgain}>
          🔁 Play Again
        </Button>
        <Button variant="outline" color="green" onClick={handleMainMenu}>
          🏠 Main Menu
        </Button>
      </Group>

      <ChatBox />
    </Stack>
  );
}
