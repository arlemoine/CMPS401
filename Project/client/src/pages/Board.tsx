// client/src/pages/Board.tsx
import { useEffect } from "react";
import { Button, Stack, Title, Text, Group, Alert } from "@mantine/core";
import { ws } from "../api/ws";
import { useParams, useNavigate } from "react-router-dom";
import { useStore } from "../state/store";

export default function Board() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const {
    me,
    players,
    board,
    turn,
    matchStatus,
    setBoard,
    setTurn,
    setMatchStatus,
    setPlayers
  } = useStore();

  useEffect(() => {
    if (!id) return;
    ws.connect();

    const unsub = ws.onMessage((msg) => {
      console.log("[Board] Received message:", msg);

      if (msg.type === "state_update" && msg.payload.matchId === id) {
        const p = msg.payload;
        setPlayers(p.players);
        setBoard(p.board || Array(9).fill(null));
        setTurn(p.turn);
        setMatchStatus(p.status);
      }

      if (msg.type === "error") {
        console.warn("[Board] error:", msg.payload);
        alert(msg.payload.message);
      }
    });

    return () => unsub();
  }, [id, setBoard, setTurn, setMatchStatus, setPlayers]);

  const handleCellClick = (index: number) => {
    if (!id || !me) return;
    if (matchStatus !== "IN_PROGRESS" || turn !== me.mark || board[index]) return;
    
    console.log(`[Board] Making move at index ${index}`);
    ws.send({ type: "make_move", payload: { matchId: id, index } });
  };

  const renderCell = (value: string | null, index: number) => {
    const isClickable = matchStatus === "IN_PROGRESS" && turn === me?.mark && !value;
    
    return (
      <Button
        key={index}
        color={value ? (value === "X" ? "blue" : "red") : "gray"}
        variant={value ? "filled" : "outline"}
        style={{ 
          width: 80, 
          height: 80, 
          fontSize: 24, 
          borderRadius: 8,
          cursor: isClickable ? 'pointer' : 'default'
        }}
        onClick={() => isClickable && handleCellClick(index)}
        disabled={!isClickable}
      >
        {value || ""}
      </Button>
    );
  };

  const winner = checkWinner(board);
  const isDraw = !winner && board.every(cell => cell !== null);

  return (
    <Stack align="center" mt="lg">
      <Title order={2}>Tic-Tac-Toe</Title>
      <Text>Match ID: {id}</Text>

      {matchStatus === "WAITING" && (
        <Alert color="yellow">Waiting for both players to join...</Alert>
      )}

      {matchStatus === "IN_PROGRESS" && (
        <>
          <Group mt="sm">
            {players.map((p) => (
              <Text key={p.id} fw={p.mark === turn ? 700 : 400}>
                {p.displayName} ({p.mark}) {p.id === me?.id && "(You)"} {p.mark === turn && "⬅️ Turn"}
              </Text>
            ))}
          </Group>
          
          {me && (
            <Text>
              Your turn: {turn === me.mark ? "Yes" : "No"} (You are {me.mark})
            </Text>
          )}

          <div
            style={{
              display: "grid",
              gridTemplateColumns: "repeat(3, 80px)",
              gap: 8,
              marginTop: 20,
            }}
          >
            {board.map(renderCell)}
          </div>
        </>
      )}

      {(matchStatus === "FINISHED" || winner || isDraw) && (
        <Stack align="center" mt="lg">
          {winner && (
            <Alert color="green">
              🎉 Winner: {players.find((p) => p.mark === winner)?.displayName ?? winner}!
            </Alert>
          )}
          {isDraw && (
            <Alert color="yellow">It's a draw! No winner.</Alert>
          )}
        </Stack>
      )}

      <Button mt="xl" color="red" onClick={() => navigate("/")}>
        Leave Match
      </Button>
    </Stack>
  );
}

function checkWinner(board: (string | null)[]): string | null {
  const wins = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
    [0, 3, 6], [1, 4, 7], [2, 5, 8], // columns
    [0, 4, 8], [2, 4, 6] // diagonals
  ];
  
  for (const [a, b, c] of wins) {
    if (board[a] && board[a] === board[b] && board[b] === board[c]) {
      return board[a];
    }
  }
  return null;
}