// client/src/pages/Board.tsx
import { useEffect } from "react";
import { Button, Stack, Title, Text, Alert, Grid, Badge, Group } from "@mantine/core";
import { useParams, useNavigate } from "react-router-dom";
import { ws } from "../api/ws";
import { useStore } from "../state/store";
import ChatBox from "../components/ChatBox";

export default function Board() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const {
    playerName,
    players,
    board,
    whosTurn,
    status,
    setBoard,
    setWhosTurn,
    setStatus,
    setPlayers,
    setGameId,
    setPlayerName,
    clearChatMessages,
  } = useStore();

  useEffect(() => {
    ws.connect();

    const unsub = ws.onMessage((msg) => {
      console.log("[Board] Received:", msg);

      if (msg.type === "TicTacToe") {
        const { board: serverBoard, whos_turn, status: gameStatus } = msg.data;

        if (serverBoard) {
          try {
            if (typeof serverBoard === "string") {
              setBoard(JSON.parse(serverBoard));
            } else if (Array.isArray(serverBoard)) {
              setBoard(serverBoard);
            }
          } catch (e) {
            console.error("[Board] Failed to parse board:", serverBoard, e);
          }
        }

        if (whos_turn) {
          console.log("[Board] Turn updated to:", whos_turn);
          setWhosTurn(whos_turn);
        }

        if (gameStatus) {
          console.log("[Board] Status updated to:", gameStatus);
          setStatus(gameStatus);
        }
      }

      if (msg.type === "GameRoom") {
        const { players: serverPlayers } = msg.data;
        if (serverPlayers && Array.isArray(serverPlayers)) {
          setPlayers(serverPlayers);
        }
      }
    });

    return () => unsub();
  }, [setBoard, setWhosTurn, setStatus, setPlayers]);

  const getPlayerMark = (player: string) => {
    const index = players.indexOf(player);
    return index === 0 ? "X" : index === 1 ? "O" : "?";
  };

  const getCellSymbol = (value: number) => {
    if (value === 1) return "X";
    if (value === -1) return "O";
    return "";
  };

  const handleMove = (row: number, col: number) => {
    if (!id || !playerName) {
      console.warn("[Board] Missing game ID or player name");
      return;
    }

    if (status.startsWith("gameover")) {
      console.warn("[Board] Game is over");
      return;
    }

    if (whosTurn !== playerName) {
      console.warn(`[Board] Not your turn (current: ${whosTurn}, you: ${playerName})`);
      return;
    }

    if (board[row][col] !== 0) {
      console.warn("[Board] Cell already occupied");
      return;
    }

    const choice = `${String.fromCharCode(65 + row)}${col + 1}`;
    console.log(`[Board] ${playerName} making move: ${choice}`);

    ws.send({
      type: "TicTacToe",
      data: {
        game_id: id,
        whos_turn: playerName,
        choice,
      },
    });
  };

  // ✅ Navigate back to match with state flag
  const handleBackToRoom = () => {
    clearChatMessages();
    console.log("[Board] Navigating back to match room");
    navigate(`/match/${id}`, { state: { fromBoard: true } });
  };

  const handleMainMenu = () => {
    console.log("[Board] Leaving game and going to main menu");
    clearChatMessages();

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

    console.log("[Board] State reset, navigating to dashboard");
    navigate("/");
  };

  const renderCell = (value: number, row: number, col: number) => {
    const symbol = getCellSymbol(value);
    const isClickable =
      whosTurn === playerName &&
      value === 0 &&
      !status.startsWith("gameover") &&
      status !== "waiting";

    return (
      <Button
        key={`${row}-${col}`}
        color={symbol === "X" ? "blue" : symbol === "O" ? "red" : "gray"}
        variant={symbol ? "filled" : "outline"}
        onClick={() => isClickable && handleMove(row, col)}
        disabled={!isClickable}
        style={{
          width: 100,
          height: 100,
          fontSize: 40,
          fontWeight: "bold",
          borderRadius: 12,
          cursor: isClickable ? "pointer" : "default",
          transition: "all 0.2s",
          border: isClickable ? "3px solid #228be6" : symbol ? "2px solid #495057" : "2px dashed #adb5bd",
          boxShadow: isClickable ? "0 4px 12px rgba(34, 139, 230, 0.4)" : "none",
        }}
      >
        {symbol}
      </Button>
    );
  };

  const isGameOver = status.startsWith("gameover");

  const getWinner = () => {
    if (status === "gameover_x") return players[0];
    if (status === "gameover_o") return players[1];
    return null;
  };

  const winner = getWinner();

  return (
    <Grid mt="lg" gutter="xl" style={{ alignItems: "flex-start" }}>
      <Grid.Col span="auto">
        <Stack align="center">
          <Title order={1} style={{ fontSize: 40 }}>
            ⭕ Tic-Tac-Toe ❌
          </Title>
          <Text size="sm" c="dimmed">
            Game ID: <strong>{id}</strong>
          </Text>

          <Group gap="lg" mt="md">
            {players.map((player, idx) => (
              <Badge
                key={idx}
                size="xl"
                variant={player === playerName ? "filled" : "light"}
                color={idx === 0 ? "blue" : "red"}
                style={{
                  padding: "12px 20px",
                  fontSize: 16,
                }}
              >
                <Group gap="xs">
                  <Text fw={700}>{getPlayerMark(player)}</Text>
                  <Text>|</Text>
                  <Text>{player}</Text>
                  {player === playerName && <Text fw={700}>← YOU</Text>}
                </Group>
              </Badge>
            ))}
          </Group>

          {status === "waiting" && (
            <Alert color="yellow" mt="md" style={{ width: "100%", maxWidth: 400 }}>
              ⏳ Waiting for opponent to join...
            </Alert>
          )}

          {!isGameOver && whosTurn && status !== "waiting" && (
            <Alert
              color={whosTurn === playerName ? "blue" : "gray"}
              mt="md"
              style={{ width: "100%", maxWidth: 400 }}
            >
              <Group justify="center" gap="xs">
                <Text size="lg" fw={600}>
                  {whosTurn === playerName ? "🎮 YOUR TURN!" : `⏳ ${whosTurn}'s turn`}
                </Text>
                <Text size="lg" fw={700}>
                  ({getPlayerMark(whosTurn)})
                </Text>
              </Group>
            </Alert>
          )}

          <div
            style={{
              display: "grid",
              gridTemplateColumns: "repeat(3, 100px)",
              gap: 12,
              marginTop: 30,
              padding: 20,
              backgroundColor: "rgba(0, 0, 0, 0.4)",
              borderRadius: 16,
              boxShadow: "0 8px 32px rgba(0, 0, 0, 0.5)",
            }}
          >
            {board.map((row, r) => row.map((cell, c) => renderCell(cell, r, c)))}
          </div>

          {isGameOver && (
            <Stack align="center" mt="xl" gap="md">
              <Alert
                color={winner === playerName ? "green" : "blue"}
                title="🎉 GAME OVER!"
                style={{ width: "100%", maxWidth: 500, fontSize: 18 }}
              >
                {status === "gameover_x" && (
                  <Text size="lg" fw={600}>
                    {winner === playerName
                      ? "🏆 YOU WON with X!"
                      : `${winner} won with X!`}
                  </Text>
                )}
                {status === "gameover_o" && (
                  <Text size="lg" fw={600}>
                    {winner === playerName
                      ? "🏆 YOU WON with O!"
                      : `${winner} won with O!`}
                  </Text>
                )}
                {status === "gameover_draw" && (
                  <Text size="lg" fw={600}>
                    🤝 It's a DRAW! No winner.
                  </Text>
                )}
              </Alert>

              <Group mt="md">
                <Button
                  size="lg"
                  color="blue"
                  onClick={handleBackToRoom}
                >
                  ← Back to Room
                </Button>
                <Button
                  size="lg"
                  color="green"
                  variant="outline"
                  onClick={handleMainMenu}
                >
                  🏠 Main Menu
                </Button>
              </Group>
            </Stack>
          )}

          {!isGameOver && (
            <Button
              mt="xl"
              size="md"
              color="red"
              variant="outline"
              onClick={handleMainMenu}
            >
              ❌ Leave Game
            </Button>
          )}
        </Stack>
      </Grid.Col>

      <Grid.Col span="content">
        <ChatBox />
      </Grid.Col>
    </Grid>
  );
}