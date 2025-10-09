import { useEffect, useState } from "react";
import { Container, Title } from "@mantine/core";
import { BrowserRouter, Routes, Route, useNavigate } from "react-router-dom";
import { ws } from "./api/ws";
import CreateJoin from "./pages/CreateJoin";
import Match from "./pages/Match";

function AppRoutes() {
  const navigate = useNavigate();
  const [status, setStatus] = useState("disconnected");
  const [_serverVersion, setServerVersion] = useState<string | undefined>(undefined);

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
          navigate(`/match/${id}`);
          break;
        }
        case "error":
          // TODO: surface via Mantine notifications
          console.warn("server error", msg.payload);
          break;
      }
    });

    return () => {
      offOpen();
      offClose();
      offMsg();
    };
  }, [navigate]);

  return (
    <Routes>
      <Route path="/" element={<CreateJoin />} />
      <Route path="/match/:id" element={<Match />} />
    </Routes>
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
