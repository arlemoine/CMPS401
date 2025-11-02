// client/src/App.tsx
import { useEffect, useState, type JSX } from "react";
import { Container, Title, Alert, Button, Group } from "@mantine/core";
import { BrowserRouter, Routes, Route, useNavigate, Navigate, useLocation } from "react-router-dom";
import { ws } from "./api/ws";
import { useStore } from "./state/store";
import CreateJoin from "./pages/CreateJoin";
import Match from "./pages/Match";
import Board from "./pages/Board";
import bg from "./assets/bg20.jpg";
import { auth } from "./firebase"; // ✅ import firebase auth
import { onAuthStateChanged, signOut } from "firebase/auth";
import Login from "./components/Login";
import Signup from "./components/Signup";
import Dashboard from "./components/Dashboard";


// ---------------------- PROTECTED ROUTES ----------------------
function ProtectedRoute({ children }: { children: JSX.Element }) {
  const [user, setUser] = useState(auth.currentUser);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (u) => {
      setUser(u);
      setLoading(false);
    });
    return unsub;
  }, []);

  if (loading) return <p style={{ color: "white", textAlign: "center" }}>Checking authentication...</p>;

  return user ? children : <Navigate to="/login" />;
}

// ---------------------- APP ROUTES ----------------------
function AppRoutes({ user }: { user: any }) {
  const navigate = useNavigate();
  const [error, setError] = useState<string>("");
  const [status, setStatus] = useState<"connected" | "disconnected">("disconnected");

  const { setMatchId, setPlayers, setMatchStatus, setMe, setBoard, setTurn } = useStore();

  // 🔗 Connect to backend WS
  useEffect(() => {
    ws.connect();

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

          if (status === "IN_PROGRESS" && window.location.pathname.includes("/match/")) {
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
      {status === "disconnected" && <Alert color="yellow" mb="md">Connecting to server...</Alert>}

      <Routes>
        {/* 🧭 Auth Routes */}
        <Route path="/login" element={<Login />} />
        <Route path="/signup" element={<Signup />} />
        <Route path="/" element={user ? <Dashboard /> : <Navigate to="/login" />} />

        {/* 🧭 Protected Game Routes */}
        <Route path="/createjoin" element={<ProtectedRoute><CreateJoin /></ProtectedRoute>} />
        <Route path="/match/:id" element={<ProtectedRoute><Match /></ProtectedRoute>} />
        <Route path="/board/:id" element={<ProtectedRoute><Board /></ProtectedRoute>} />

        {/* Default fallback */}
        <Route path="*" element={<Navigate to="/" />} />
      </Routes>
    </>
  );
}

// ---------------------- APP WRAPPER ----------------------
export default function App() {
  const basename = (import.meta.env.BASE_URL || "/").replace(/\/$/, "");
  const [user, setUser] = useState(auth.currentUser);
  useEffect(() => {
    const unsub = onAuthStateChanged(auth, (u) => setUser(u));
    return unsub;
  }, []);

   const handleLogout = async () => {
      await signOut(auth); 
  };
   const showLogout = !["/", "/login", "/signup"].includes(location.pathname);

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
          <div style={{
            width: "100%",
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    padding: "0.75rem 2rem",
    borderRadius: "8px",
    marginBottom: "1.5rem",
            }}>
          <Title order={2} ta="center" mb="lg">
            Multiplayer-Game Prototype
          </Title>

          {user && (
              <Button
                variant="filled"
                color="red"
                size="xs"
                onClick={handleLogout}
              >
                Logout
              </Button>
            )}
          </div>

          {/* Show logout if logged in
          {user && (
            <div style={{ textAlign: "center", marginBottom: "1rem" }}>
              <span>Signed in as {user.email}</span>
              <Button
                variant="light"
                color="red"
                size="xs"
                ml="sm"
                onClick={handleLogout}
              >
                Logout
              </Button>
            </div>
          )} */}
         <Container size="lg" style={{width:"80%"}}>
          <AppRoutes user={user} />
          </Container>
        </Container>
      </div>
    </BrowserRouter>
  );
}
