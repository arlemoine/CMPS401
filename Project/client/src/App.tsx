// client/src/App.tsx
import { useEffect, useState } from "react";
import { Container, Title, Alert } from "@mantine/core";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { ws, type ServerMsg } from "./api/ws";
import { useStore } from "./state/store";
import CreateJoin from "./pages/CreateJoin";
import Match from "./pages/Match";
import Board from "./pages/Board";
import Dashboard from "./components/Dashboard";
import bg from "./assets/bg20.jpg";

function AppRoutes() {
  const [error, setError] = useState("");
  const [status, setStatus] = useState<"connected" | "disconnected">("disconnected");

  const {
    setGameId,
    setBoard,
    setWhosTurn,
    setStatus: setGameStatus,
    addChatMessage,
    setPlayers,
  } = useStore();

  // 🔗 WebSocket Connection Setup
  useEffect(() => {
    ws.connect();

    const offOpen = ws.onOpen(() => {
      console.log("[App] WebSocket Connected ✅");
      setStatus("connected");
      setError("");
    });

    const offClose = ws.onClose((code, reason) => {
      console.warn("[App] WebSocket Disconnected ❌", code, reason);
      setStatus("disconnected");

      if (code !== 1000) {
        setError(`Connection lost: ${reason || "Unknown error"}`);
      }
    });

    const offMsg = ws.onMessage((msg: ServerMsg) => {
      console.log("[App] Global message received:", msg);

      switch (msg.type) {
        case "Echo":
          console.log("[App] Echo:", msg.data.message);
          break;

        case "GameRoom": {
          const { game_id, action, players } = msg.data;

          if (game_id) {
            setGameId(game_id);
          }

          if (players && Array.isArray(players)) {
            setPlayers(players);
            console.log(`[App] Players list updated:`, players);
          }

          console.log(`[App] GameRoom action: ${action} for game ${game_id}`);
          break;
        }

        case "TicTacToe": {
          const { board, whos_turn, status } = msg.data;
          console.log("[App] TicTacToe update:", msg.data);

          if (board) {
            try {
              if (typeof board === "string") {
                setBoard(JSON.parse(board));
              } else if (Array.isArray(board)) {
                setBoard(board);
              }
            } catch (e) {
              console.error("[App] Failed to parse board:", board, e);
            }
          }

          if (whos_turn) setWhosTurn(whos_turn);
          if (status) setGameStatus(status);
          break;
        }

        case "Chat": {
          addChatMessage(msg.data);
          break;
        }

        default:
          console.warn("[App] Unknown message type:", msg);
          break;
      }
    });

    return () => {
      offOpen();
      offClose();
      offMsg();
    };
  }, [setGameId, setBoard, setWhosTurn, setGameStatus, addChatMessage, setPlayers]);

  return (
    <>
      {/* Connection status alerts */}
      {/* {status === "disconnected" && (
        <Alert color="yellow" mb="md" style={{ textAlign: "center" }}>
          ⚠️ Connecting to server...
        </Alert>
      )}

      {error && (
        <Alert color="red" mb="md" withCloseButton onClose={() => setError("")}>
          {error}
        </Alert>
      )} */}

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
          minHeight: "100vh",
          backgroundImage: `url(${bg})`,
          backgroundSize: "cover",
          backgroundPosition: "center",
          backgroundRepeat: "no-repeat",
          backgroundAttachment: "fixed",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          color: "white",
          flexDirection: "column",
          overflow: "auto",
          padding: "20px 0",
        }}
      >
        <Container size="lg" style={{ width: "100%", maxWidth: 1200 }}>
          <div
            style={{
              width: "100%",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              padding: "1rem 2rem",
              borderRadius: "8px",
              marginBottom: "1.5rem",
              backgroundColor: "rgba(0, 0, 0, 0.6)",
              backdropFilter: "blur(10px)",
            }}
          >
            <Title order={1} ta="center">
              🎮 Multiplayer Game Prototype
            </Title>
          </div>

          <Container
            size="lg"
            style={{
              width: "95%",
              backgroundColor: "rgba(0, 0, 0, 0.7)",
              borderRadius: "12px",
              padding: "2rem",
              backdropFilter: "blur(10px)",
              boxShadow: "0 8px 32px 0 rgba(0, 0, 0, 0.37)",
            }}
          >
            <AppRoutes />
          </Container>
        </Container>
      </div>
    </BrowserRouter>
  );
}