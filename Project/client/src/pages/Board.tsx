// client/src/pages/Board.tsx
import { useEffect } from "react";
import { Button, Stack, Title, Text, Alert, Grid } from "@mantine/core";
import { useParams, useNavigate } from "react-router-dom";
import { ws } from "../api/ws";
import { useStore } from "../state/store";
import ChatBox from "../components/ChatBox";

export default function Board() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { playerName, board, whosTurn, status, setBoard, setWhosTurn, setStatus } = useStore();

  useEffect(() => {
    ws.connect();

    const unsub = ws.onMessage((msg) => {
      console.log("[Board] Received:", msg);

      if (msg.type === "TicTacToe") {
        const { board, whos_turn, status } = msg.data;
        try {
          if (board) setBoard(JSON.parse(board));
        } catch {
          console.warn("Invalid board data:", board);
        }
        if (whos_turn) setWhosTurn(whos_turn);
        if (status) setStatus(status);
      }
    });

    return () => unsub();
  }, [setBoard, setWhosTurn, setStatus]);

  const handleMove = (row: number, col: number) => {
    if (!id || !playerName) return;
    if (status.startsWith("gameover") || whosTurn !== playerName) return;

    const choice = `${String.fromCharCode(65 + row)}${col + 1}`; // e.g. A1, B2
    console.log(`[Board] Move by ${playerName}: ${choice}`);

    ws.send({
      type: "TicTacToe",
      data: {
        whos_turn: playerName,
        choice,
      },
    });
  };

  const renderCell = (value: number, row: number, col: number) => {
    const symbol = value === 1 ? "X" : value === -1 ? "O" : "";
    const isClickable = whosTurn === playerName && value === 0 && !status.startsWith("gameover");

    return (
      <Button
        key={`${row}-${col}`}
        color={symbol === "X" ? "blue" : symbol === "O" ? "red" : "gray"}
        variant={symbol ? "filled" : "outline"}
        onClick={() => isClickable && handleMove(row, col)}
        disabled={!isClickable}
        style={{
          width: 90,
          height: 90,
          fontSize: 24,
          borderRadius: 8,
          cursor: isClickable ? "pointer" : "default",
        }}
      >
        {symbol}
      </Button>
    );
  };

  const isGameOver = status.startsWith("gameover");

  return (
    <Grid mt="lg" gutter="xl" style={{ alignItems: "flex-start" }}>
      <Grid.Col span="auto">
        <Stack align="center">
          <Title order={2}>Tic-Tac-Toe</Title>
          <Text>Game ID: {id}</Text>

          {status === "waiting" && <Alert color="yellow">Waiting for opponent...</Alert>}
          {!isGameOver && whosTurn && <Text>Turn: {whosTurn}</Text>}

          <div
            style={{
              display: "grid",
              gridTemplateColumns: "repeat(3, 90px)",
              gap: 8,
              marginTop: 20,
            }}
          >
            {board.map((row, r) =>
              row.map((cell, c) => renderCell(cell, r, c))
            )}
          </div>

          {isGameOver && (
            <Stack align="center" mt="lg">
              <Alert color="green">
                {status === "gameover_x"
                  ? "🎉 Player X wins!"
                  : status === "gameover_o"
                  ? "🎉 Player O wins!"
                  : "It's a tie!"}
              </Alert>
              <Button mt="md" color="blue" onClick={() => navigate(`/match/${id}`)}>
                Back to Room
              </Button>
            </Stack>
          )}
        </Stack>
      </Grid.Col>

      {/* Chat section (right side) */}
      <Grid.Col span="content">
        <ChatBox />
      </Grid.Col>
    </Grid>
  );
}
