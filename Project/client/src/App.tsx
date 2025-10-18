// client/src/App.tsx
import { useEffect, useState } from "react";
import { Container, Title, Alert } from "@mantine/core";
import { BrowserRouter, Routes, Route, useNavigate } from "react-router-dom";
import { ws } from "./api/ws";
import { useStore } from "./state/store";
import CreateJoin from "./pages/CreateJoin";
import Match from "./pages/Match";
import Board from "./pages/Board";
import bg from "./assets/bg9.jpg";

function AppRoutes() {
  const navigate = useNavigate();
  const [error, setError] = useState<string>("");
  const [status, setStatus] = useState<"connected" | "disconnected">("disconnected");

  const { setMatchId, setPlayers, setMatchStatus, setMe, setBoard, 
    setTurn  } = useStore();

  useEffect(() => {
    ws.connect();

    // Handlers for WS events
    const offOpen = ws.onOpen(() => {
      console.log("[WS] Connected");
      setStatus("connected");
    });

    const offClose = ws.onClose((code, reason) => {
      console.log("[WS] Disconnected", code, reason);
      setStatus("disconnected");
    });

    const offMsg = ws.onMessage((msg) => {
      switch (msg.type) {
        case "hello":
          console.log("[WS] Server version:", msg.payload.serverVersion);
          break;

        case "match_created": {
          const { matchId, you } = msg.payload;
          setMatchId(matchId);
          setMe(you);
          console.log("[App] Match created, navigating to:", matchId);
          navigate(`/match/${matchId}`);
          break;
        }

        case "joined_match": {
          const { matchId, you } = msg.payload;
          setMatchId(matchId);
          setMe(you);
           console.log("[App] Joined match, navigating to:", matchId);
          navigate(`/match/${matchId}`);
          break;
        }

        case "state_update": {
        const { matchId, players, status, board, turn } = msg.payload;
        console.log("[App] State update:", { matchId, status, players: players.length });
        
        setMatchId(matchId);
        setPlayers(players);
        setMatchStatus(status);
        setBoard(board || Array(9).fill(null));
        setTurn(turn);
        
        // If we're on the match page and game starts, navigate to board
        if (status === "IN_PROGRESS" && window.location.pathname.includes('/match/')) {
          console.log("[App] Game started, navigating to board");
          navigate(`/board/${matchId}`);
        }
        break;
      }

        case "error":
          console.warn("[WS] Server error", msg.payload);
          setError(`${msg.payload.code}: ${msg.payload.message}`);
          setTimeout(() => setError(""), 5000);
          break;
      }
    });

    return () => {
      offOpen();
      offClose();
      offMsg();
    };
  }, [navigate, setMatchId, setPlayers, setMatchStatus, setMe, setBoard, setTurn]);

  return (
    <>
      {error && <Alert color="red" mb="md">{error}</Alert>}
      {status === "disconnected" && (
        <Alert color="yellow" mb="md">Connecting to server...</Alert>
      )}
      <Routes>
        <Route path="/" element={<CreateJoin />} />
        <Route path="/match/:id" element={<Match />} />
        <Route path="/board/:id" element={<Board />} />
      </Routes>
    </>
  );
}

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
        backgroundRepeat: "no-repeat",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        color: "white",
      }}
    >
      <Container size="sm" style={{ paddingTop: 40 }}>
        <Title order={2} ta="center" mb="lg">
          Tic-Tac-Toe Prototype
        </Title>
        <AppRoutes />
      </Container>
      </div>
    </BrowserRouter>
  );
}
