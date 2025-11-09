// client/src/pages/RockPaperScissors.tsx
import { useEffect, useState } from "react";
import { Button, Stack, Title, Text, Alert, Grid, Badge, Group, Paper } from "@mantine/core";
import { useParams, useNavigate } from "react-router-dom";
import { ws } from "../api/ws";
import { useStore } from "../state/store";
import ChatBox from "../components/ChatBox";

type Choice = "rock" | "paper" | "scissors" | null;

interface RpsGameState {
  player1: string | null;
  player2: string | null;
  player1_choice: string | null;
  player2_choice: string | null;
  status: string;
  winner: string | null;
  message: string | null;
}

export default function RockPaperScissors() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { 
    playerName, 
    players, 
    setPlayers, 
    setGameId, 
    setPlayerName, 
    clearChatMessages, 
    addChatMessage 
  } = useStore();

  const [gameState, setGameState] = useState<RpsGameState>({
    player1: null,
    player2: null,
    player1_choice: null,
    player2_choice: null,
    status: "waiting_for_opponent",
    winner: null,
    message: null,
  });

  const [myChoice, setMyChoice] = useState<Choice>(null);
  const [isWaitingForOpponent, setIsWaitingForOpponent] = useState(false);

  useEffect(() => {
    ws.connect();

    const unsub = ws.onMessage((msg) => {
      console.log("[RPS] Received:", msg);

      if (msg.type === "RockPaperScissors") {
        const data = msg.data;
        setGameState({
          player1: data.player1 || null,
          player2: data.player2 || null,
          player1_choice: data.player1_choice || null,
          player2_choice: data.player2_choice || null,
          status: data.status || "waiting_for_opponent",
          winner: data.winner || null,
          message: data.message || null,
        });

        // Check if both choices are revealed (round complete)
        if (data.status === "round_complete" && data.player1_choice && data.player2_choice) {
          setIsWaitingForOpponent(false);
        }
      }

      if (msg.type === "GameRoom") {
        const { players: serverPlayers } = msg.data;
        if (serverPlayers && Array.isArray(serverPlayers)) {
          setPlayers(serverPlayers);
        }
      }

      if (msg.type === "Chat") {
        // Chat messages handled by ChatBox component
        console.log("[RPS] Chat message received:", msg.data);
        addChatMessage({
          player_name: msg.data.player_name,
          chat_message: msg.data.chat_message,
          time: msg.data.time,
        });
      }
    });

    // Request initial game state when component mounts
    if (id && playerName) {
      console.log("[RPS] Requesting initial game state");
      ws.send({
        type: "RockPaperScissors",
        data: {
          game_id: id,
          player_name: playerName,
        },
      });
    }

    return () => unsub();
  }, [id, playerName, setPlayers]);

  const handleChoice = (choice: Choice) => {
    if (!id || !playerName || !choice) {
      console.warn("[RPS] Missing game ID, player name, or choice");
      return;
    }

    if (gameState.status === "round_complete") {
      console.log("[RPS] Starting new round");
      setMyChoice(null);
      setIsWaitingForOpponent(false);
    }

    console.log(`[RPS] ${playerName} chose ${choice}`);
    setMyChoice(choice);
    setIsWaitingForOpponent(true);

    ws.send({
      type: "RockPaperScissors",
      data: {
        game_id: id,
        player_name: playerName,
        choice: choice,
      },
    });
  };

  const handleBackToRoom = () => {
    if (!id) return;
    
    clearChatMessages();
    console.log("[RPS] Navigating back to match room");
    navigate(`/match/${id}`, { state: { fromBoard: true } });
  };

  const handleMainMenu = () => {
    console.log("[RPS] Leaving game and going to main menu");
    clearChatMessages();

    if (id && playerName) {
      ws.send({
        type: "GameRoom",
        data: {
          game: "rockpaperscissors",
          action: "leave",
          player_name: playerName,
          game_id: id,
        },
      });
    }

    setGameId(null);
    setPlayers([]);
    setPlayerName("");
    sessionStorage.removeItem("ttt_playerName");
    ws.close();

    console.log("[RPS] State reset, navigating to dashboard");
    navigate("/");
  };

  const getChoiceEmoji = (choice: string | null) => {
    if (!choice) return "❓";
    switch (choice.toLowerCase()) {
      case "rock":
        return "🪨";
      case "paper":
        return "📄";
      case "scissors":
        return "✂️";
      default:
        return "❓";
    }
  };

  const isRoundComplete = gameState.status === "round_complete";
  const isWaiting = gameState.status === "waiting_for_opponent";
  const canMakeChoice = !isWaitingForOpponent && !isRoundComplete && !isWaiting;

  const getWinnerDisplay = () => {
    if (!gameState.winner) return null;
    
    if (gameState.winner === "tie") {
      return (
        <Alert color="yellow" title="🤝 It's a Tie!" style={{ width: "100%", maxWidth: 500, fontSize: 18 }}>
          <Text size="lg" fw={600}>
            Both players chose {getChoiceEmoji(gameState.player1_choice)} - Try again!
          </Text>
        </Alert>
      );
    }

    const isWinner = gameState.winner === playerName;
    return (
      <Alert
        color={isWinner ? "green" : "red"}
        title={isWinner ? "🏆 YOU WON!" : "💔 YOU LOST!"}
        style={{ width: "100%", maxWidth: 500, fontSize: 18 }}
      >
        <Text size="lg" fw={600}>
          {gameState.message || `${gameState.winner} wins this round!`}
        </Text>
      </Alert>
    );
  };

  return (
    <Grid mt="lg" gutter="xl" style={{ alignItems: "flex-start" }}>
      <Grid.Col span="auto">
        <Stack align="center">
          <Title order={1} style={{ fontSize: 40 }}>
            🪨 Rock Paper Scissors ✂️
          </Title>
          <Text size="sm" c="dimmed">
            Game ID: <strong>{id}</strong>
          </Text>

          <Group gap="lg" mt="md">
            {gameState.player1 && (
              <Badge
                size="xl"
                variant={gameState.player1 === playerName ? "filled" : "light"}
                color="blue"
                style={{ padding: "12px 20px", fontSize: 16 }}
              >
                <Group gap="xs">
                  <Text>{gameState.player1}</Text>
                  {gameState.player1 === playerName && <Text fw={700}>← YOU</Text>}
                </Group>
              </Badge>
            )}
            {gameState.player2 && (
              <Badge
                size="xl"
                variant={gameState.player2 === playerName ? "filled" : "light"}
                color="red"
                style={{ padding: "12px 20px", fontSize: 16 }}
              >
                <Group gap="xs">
                  <Text>{gameState.player2}</Text>
                  {gameState.player2 === playerName && <Text fw={700}>← YOU</Text>}
                </Group>
              </Badge>
            )}
          </Group>

          {isWaiting && (
            <Alert color="yellow" mt="md" style={{ width: "100%", maxWidth: 400 }}>
              ⏳ Waiting for opponent to join...
            </Alert>
          )}

          {!isWaiting && !isRoundComplete && (
            <Alert
              color={canMakeChoice ? "blue" : "orange"}
              mt="md"
              style={{ width: "100%", maxWidth: 400 }}
            >
              <Text size="lg" fw={600} ta="center">
                {canMakeChoice
                  ? "🎮 Make Your Choice!"
                  : myChoice
                  ? `✅ You chose ${getChoiceEmoji(myChoice)} - Waiting for opponent...`
                  : "⏳ Waiting for opponent's choice..."}
              </Text>
            </Alert>
          )}

          {/* Choice Buttons */}
          <div
            style={{
              display: "flex",
              gap: 20,
              marginTop: 40,
              flexWrap: "wrap",
              justifyContent: "center",
            }}
          >
            {(["rock", "paper", "scissors"] as const).map((choice) => (
              <Paper
                key={choice}
                shadow="md"
                radius="lg"
                style={{
                  width: 140,
                  height: 140,
                  display: "flex",
                  flexDirection: "column",
                  alignItems: "center",
                  justifyContent: "center",
                  cursor: canMakeChoice ? "pointer" : "not-allowed",
                  backgroundColor:
                    myChoice === choice
                      ? "rgba(34, 139, 230, 0.3)"
                      : "rgba(0, 0, 0, 0.4)",
                  border:
                    myChoice === choice
                      ? "3px solid #228be6"
                      : "2px solid rgba(255, 255, 255, 0.2)",
                  transition: "all 0.3s ease",
                  opacity: canMakeChoice ? 1 : 0.5,
                }}
                onClick={() => canMakeChoice && handleChoice(choice)}
                onMouseEnter={(e) => {
                  if (canMakeChoice) {
                    e.currentTarget.style.transform = "scale(1.1)";
                    e.currentTarget.style.boxShadow = "0 8px 24px rgba(34, 139, 230, 0.4)";
                  }
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.transform = "scale(1)";
                  e.currentTarget.style.boxShadow = "none";
                }}
              >
                <Text style={{ fontSize: 60 }}>{getChoiceEmoji(choice)}</Text>
                <Text fw={600} size="lg" mt="xs" style={{ textTransform: "capitalize" }}>
                  {choice}
                </Text>
              </Paper>
            ))}
          </div>

          {/* Show choices after round is complete */}
          {isRoundComplete && gameState.player1_choice && gameState.player2_choice && (
            <Stack align="center" mt="xl" gap="md" style={{ width: "100%" }}>
              <Paper
                shadow="lg"
                radius="md"
                p="xl"
                style={{
                  backgroundColor: "rgba(0, 0, 0, 0.5)",
                  width: "100%",
                  maxWidth: 500,
                }}
              >
                <Title order={3} ta="center" mb="lg">
                  Round Results
                </Title>
                <Group justify="space-around" mb="lg">
                  <Stack align="center" gap="xs">
                    <Text fw={600}>{gameState.player1}</Text>
                    <Text style={{ fontSize: 80 }}>
                      {getChoiceEmoji(gameState.player1_choice)}
                    </Text>
                    <Text size="sm" c="dimmed" style={{ textTransform: "capitalize" }}>
                      {gameState.player1_choice}
                    </Text>
                  </Stack>

                  <Text size="xl" fw={700} c="dimmed">
                    VS
                  </Text>

                  <Stack align="center" gap="xs">
                    <Text fw={600}>{gameState.player2}</Text>
                    <Text style={{ fontSize: 80 }}>
                      {getChoiceEmoji(gameState.player2_choice)}
                    </Text>
                    <Text size="sm" c="dimmed" style={{ textTransform: "capitalize" }}>
                      {gameState.player2_choice}
                    </Text>
                  </Stack>
                </Group>

                {getWinnerDisplay()}
              </Paper>

              <Group mt="md">
                <Button size="lg" color="blue" variant="outline" onClick={handleBackToRoom}>
                  ← Back to Room
                </Button>
                <Button size="lg" color="red" variant="outline" onClick={handleMainMenu}>
                  🏠 Main Menu
                </Button>
              </Group>
            </Stack>
          )}

          {!isRoundComplete && (
            <Group mt="xl">
              <Button size="md" color="red" variant="outline" onClick={handleMainMenu}>
                ❌ Leave Game
              </Button>
            </Group>
          )}
        </Stack>
      </Grid.Col>

      <Grid.Col span="content">
        <ChatBox />
      </Grid.Col>
    </Grid>
  );
}