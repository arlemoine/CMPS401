import { useEffect, useState } from "react";
import { Container, Title, Alert } from "@mantine/core";
import { BrowserRouter, Routes, Route, useNavigate } from "react-router-dom";
import { ws } from "./api/ws";
import { useStore } from "./state/store";
import CreateJoin from "./pages/CreateJoin";
import Match from "./pages/Match";

function AppRoutes() {
  const navigate = useNavigate();
  const [status, setStatus] = useState("disconnected");
  const [_serverVersion, setServerVersion] = useState<string | undefined>(undefined);
  const [error, setError] = useState<string>("");
  const { setMatchId, setPlayers, setMatchStatus, setMe } = useStore();

  useEffect(() => {
    ws.connect();

    const offOpen = ws.onOpen(() => setStatus("open"));
    const offClose = ws.onClose(() => setStatus("disconnected"));
    const offMsg = ws.onMessage((msg) => {
      switch (msg.type) {
        case "hello":
          setServerVersion(msg.payload.serverVersion);
          break;
        case "match_created": {
          const id = msg.payload.matchId;
          setMatchId(id);
          // Navigation will happen via useEffect in CreateJoin
          break;
        }
        case "joined_match": {
          const { matchId, you } = msg.payload;
          setMatchId(matchId);
          setMe(you);
          break;
        }
        case "state_update": {
          const { matchId, players, status } = msg.payload;
          setMatchId(matchId);
          setPlayers(players);
          setMatchStatus(status);
          break;
        }
        case "error":
          console.warn("server error", msg.payload);
          setError(`${msg.payload.code}: ${msg.payload.message}`);
          setTimeout(() => setError(""), 5000); // Clear after 5s
          break;
      }
    });

    return () => {
      offOpen();
      offClose();
      offMsg();
    };
  }, [navigate, setMatchId, setPlayers, setMatchStatus, setMe]);

  return (
    <>
      {error && <Alert color="red" mb="md">{error}</Alert>}
      {status === "disconnected" && (
        <Alert color="yellow" mb="md">Connecting to server...</Alert>
      )}
      <Routes>
        <Route path="/" element={<CreateJoin />} />
        <Route path="/match/:id" element={<Match />} />
      </Routes>
    </>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <Container size="sm" style={{ paddingTop: 40 }}>
        <Title order={2} ta="center" mb="lg">
          Tic-Tac-Toe Prototype
        </Title>
        <AppRoutes />
      </Container>
    </BrowserRouter>
  );
}
