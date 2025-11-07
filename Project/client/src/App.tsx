// client/src/App.tsx
import { useEffect, useState } from "react";
import { Container, Title, Alert } from "@mantine/core";
import { BrowserRouter, Routes, Route, useNavigate, Navigate } from "react-router-dom";
import { ws, type ServerMsg } from "./api/ws";
import { useStore } from "./state/store";
import CreateJoin from "./pages/CreateJoin";
import Match from "./pages/Match";
import Board from "./pages/Board";
import Dashboard from "./components/Dashboard";
import bg from "./assets/bg20.jpg";

function AppRoutes() {
  const navigate = useNavigate();
  const [error, setError] = useState("");
  const [status, setStatus] = useState<"connected" | "disconnected">("disconnected");

  const {
    setGameId,
    setBoard,
    setWhosTurn,
    setStatus: setGameStatus,
    setPlayerName,
    addChatMessage,
  } = useStore();

  // 🔗 WebSocket Connection Setup
  useEffect(() => {
    ws.connect();

    const offOpen = ws.onOpen(() => {
      console.log("[WS] Connected ✅");
      setStatus("connected");
    });

    const offClose = ws.onClose((code, reason) => {
      console.warn("[WS] Disconnected ❌", code, reason);
      setStatus("disconnected");
    });

    const offMsg = ws.onMessage((msg: ServerMsg) => {
      console.log("[WS] Message received:", msg);

      switch (msg.type) {
        // ✅ Echo (for testing)
        case "Echo":
          console.log("[WS] Echo:", msg.data.message);
          break;

        // ✅ GameRoom events
        case "GameRoom": {
          const { game_id, action, player_name } = msg.data;
          if (action === "join") {
            setGameId(game_id);
            setPlayerName(player_name);
            console.log(`[GameRoom] Joined game ${game_id} as ${player_name}`);
            navigate(`/match/${game_id}`);
          }
          break;
        }

        // ✅ TicTacToe state updates
        case "TicTacToe": {
          const { board, whos_turn, status } = msg.data;
          console.log("[TicTacToe] Game update:", msg.data);
          setBoard(board);
          setWhosTurn(whos_turn);
          setGameStatus(status);
          break;
        }

        // ✅ Chat messages
        case "Chat": {
          addChatMessage(msg.data);
          break;
        }

        // ⚠️ Unknown type
        default:
          console.warn("[WS] Unknown message type:", msg);
          break;
      }
    });

    return () => {
      offOpen();
      offClose();
      offMsg();
    };
  }, [navigate, setGameId, setBoard, setWhosTurn, setGameStatus, setPlayerName, addChatMessage]);

  return (
    <>
      {error && <Alert color="red" mb="md">{error}</Alert>}
      {status === "disconnected" && (
        <Alert color="yellow" mb="md">
          Connecting to server...
        </Alert>
      )}

      <Routes>
        {/* 🧭 Default route */}
        <Route path="/" element={<Dashboard />} />

        {/* 🧭 Game Routes */}
        <Route path="/createjoin" element={<CreateJoin />} />
        <Route path="/match/:id" element={<Match />} />
        <Route path="/board/:id" element={<Board />} />

        {/* Fallback */}
        <Route path="*" element={<Navigate to="/" />} />
      </Routes>
    </>
  );
}

// ---------------------- APP WRAPPER ----------------------
export default function App() {
  const basename = (import.meta.env.BASE_URL || "/").replace(/\/$/, "");

  return (
    <BrowserRouter basename={basename}>
      <div
        style={{
          width: "100vw",
          height: "100vh",
          backgroundImage: `url(${bg})`,
          backgroundSize: "cover",
          backgroundPosition: "center",
          backgroundRepeat: "inherit",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          color: "white",
          flexDirection: "column",
        }}
      >
        <Container size="lg" style={{ width: "100%" }}>
          <div
            style={{
              width: "100%",
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              padding: "0.75rem 2rem",
              borderRadius: "8px",
              marginBottom: "1.5rem",
            }}
          >
            <Title order={2} ta="center" mb="lg">
              Multiplayer-Game Prototype
            </Title>
          </div>

          <Container size="lg" style={{ width: "80%" }}>
            <AppRoutes />
          </Container>
        </Container>
      </div>
    </BrowserRouter>
  );
}
