// client/src/pages/Match.tsx
import { useEffect, useRef } from "react";
import { useParams, useNavigate, useLocation } from "react-router-dom";
import { Stack, Title, Text, Badge, Group, Button } from "@mantine/core";
import { useStore } from "../state/store";
import { ws } from "../api/ws";
import ChatBox from "../components/ChatBox";

export default function Match() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const location = useLocation();
  const hasJoined = useRef(false);
  const hasNavigated = useRef(false);
  
  // ✅ Track if user came from board (back button)
  const cameFromBoard = useRef(false);

  const {
    playerName,
    gameId,
    setGameId,
    players,
    setPlayers,
    setBoard,
    setWhosTurn,
    status,
    setStatus,
    addChatMessage,
    setPlayerName,
  } = useStore();

  // Sync gameId from URL
  useEffect(() => {
    if (id && id !== gameId) setGameId(id);
  }, [id, gameId, setGameId]);

  // ✅ Detect if coming from board page (back button pressed)
  useEffect(() => {
    const fromBoard = location.state?.fromBoard === true;
    if (fromBoard) {
      cameFromBoard.current = true;
      console.log("[Match] Detected back navigation from board");
    }
  }, [location]);

  // ✅ Auto-navigate when 2 players are ready (but NOT if came from board)
  useEffect(() => {
    if (
      players.length === 2 && 
      !hasNavigated.current && 
      !cameFromBoard.current && // ✅ Don't auto-navigate if user pressed back
      id
    ) {
      hasNavigated.current = true;
      console.log("[Match] 2 players detected, starting game...");
      setStatus("IN_PROGRESS");

      setTimeout(() => {
        console.log("[Match] Navigating to board...");
        navigate(`/board/${id}`, { replace: false }); // Don't replace history
      }, 1000);
    }
  }, [players.length, id, navigate, setStatus]);

  useEffect(() => {
    ws.connect();

    const unsub = ws.onMessage((msg) => {
      console.log("[Match] Received:", msg);

      if (msg.type === "TicTacToe") {
        const { board, whos_turn, status: gameStatus } = msg.data;

        if (board) {
          try {
            if (typeof board === "string") {
              setBoard(JSON.parse(board));
            } else if (Array.isArray(board)) {
              setBoard(board);
            }
          } catch (e) {
            console.error("[Match] Failed to parse board:", board, e);
          }
        }

        if (whos_turn) setWhosTurn(whos_turn);
        if (gameStatus) setStatus(gameStatus);

        // ✅ Only auto-navigate if not came from board
        if (
          gameStatus === "IN_PROGRESS" && 
          !hasNavigated.current && 
          !cameFromBoard.current
        ) {
          hasNavigated.current = true;
          console.log("[Match] Game status IN_PROGRESS, navigating to board");
          navigate(`/board/${id}`);
        }
      }

      if (msg.type === "GameRoom") {
        const { action, game_id, players: serverPlayers } = msg.data;

        if (action === "join") {
          setGameId(game_id);

          if (serverPlayers && Array.isArray(serverPlayers)) {
            console.log("[Match] Players updated:", serverPlayers);
            setPlayers(serverPlayers);
          }
        }

        if (action === "reset") {
          hasNavigated.current = false;
          cameFromBoard.current = false; // ✅ Reset on game reset
          setBoard([
            [0, 0, 0],
            [0, 0, 0],
            [0, 0, 0],
          ]);
          setStatus("waiting");
          setWhosTurn("");
        }

        if (action === "leave") {
          hasNavigated.current = false;
          cameFromBoard.current = false; // ✅ Reset on leave
        }
      }

      if (msg.type === "Chat") {
        addChatMessage(msg.data);
      }
    });

    if (id && playerName && !hasJoined.current) {
      hasJoined.current = true;
      console.log(`[Match] ${playerName} joining game ${id}`);

      ws.send({
        type: "GameRoom",
        data: {
          game: "tictactoe",
          action: "join",
          player_name: playerName,
          game_id: id,
        },
      });
    }

    return () => {
      unsub();
    };
  }, [
    id,
    playerName,
    setBoard,
    setWhosTurn,
    setStatus,
    setGameId,
    setPlayers,
    addChatMessage,
    navigate,
  ]);

  const handleGoToBoard = () => {
    if (players.length === 2) {
      cameFromBoard.current = false; // ✅ Reset flag when manually navigating
      navigate(`/board/${id}`);
    } else {
      console.warn("[Match] Cannot go to board, waiting for 2 players");
    }
  };

  const handlePlayAgain = () => {
    if (!id) return;

    hasNavigated.current = false;
    cameFromBoard.current = false; // ✅ Reset flag

    ws.send({
      type: "GameRoom",
      data: {
        game: "tictactoe",
        action: "reset",
        player_name: playerName,
        game_id: id,
      },
    });

    setBoard([
      [0, 0, 0],
      [0, 0, 0],
      [0, 0, 0],
    ]);
    setStatus("waiting");
    setWhosTurn("");
  };

  const handleMainMenu = () => {
    console.log("[Match] Leaving game and going to main menu");

    if (id && playerName) {
      ws.send({
        type: "GameRoom",
        data: {
          game: "tictactoe",
          action: "leave",
          player_name: playerName,
          game_id: id,
        },
      });
    }

    setGameId(null);
    setPlayers([]);
    setBoard([
      [0, 0, 0],
      [0, 0, 0],
      [0, 0, 0],
    ]);
    setWhosTurn("");
    setStatus("waiting");
    setPlayerName("");

    sessionStorage.removeItem("ttt_playerName");
    ws.close();

    console.log("[Match] State reset, navigating to dashboard");
    navigate("/");
  };

  const matchStatus = status.startsWith("gameover")
    ? "FINISHED"
    : status === "IN_PROGRESS"
    ? "IN_PROGRESS"
    : "WAITING";

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
        {players.length === 2 && (
          <Text c="green" fw={600}>
            ✅ Ready to start!
          </Text>
        )}
      </Stack>

      <Group mt="lg" justify="center">
        <Button
          variant="filled"
          color="blue"
          onClick={handleGoToBoard}
          disabled={players.length < 2}
        >
          🎮 Go to Game Board
        </Button>
        <Button variant="filled" color="teal" onClick={handlePlayAgain}>
          🔁 Play Again
        </Button>
        <Button variant="outline" color="red" onClick={handleMainMenu}>
          🏠 Main Menu
        </Button>
      </Group>

      <ChatBox />
    </Stack>
  );
}