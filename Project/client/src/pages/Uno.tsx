// client/src/pages/Uno.tsx
import { useEffect, useState } from "react";
import { Button, Stack, Title, Text, Grid, Badge, Group, Paper, Modal, SimpleGrid, Alert } from "@mantine/core";
import { useParams, useNavigate } from "react-router-dom";
import { ws, type UnoCard } from "../api/ws";
import { useStore } from "../state/store";
import ChatBox from "../components/ChatBox";

interface UnoGameState {
  players: string[];
  current_idx: number;
  direction: number;
  top_discard: UnoCard | null;
  chosen_color: string | null;
  pending_draw: number;
  public_counts: number[];
  hand: UnoCard[];
  winner: string | null;
  gameStarted: boolean;
}

const COLOR_MAP: { [key: string]: string } = {
  Red: "#EF4444",
  Yellow: "#EAB308",
  Green: "#22C55E",
  Blue: "#3B82F6",
  Wild: "#6B7280",
};

export default function Uno() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { 
    playerName, 
    players, 
    setPlayers, 
    setGameId, 
    setPlayerName, 
    clearChatMessages,
    addChatMessage // ✅ Add this to handle incoming chat messages
  } = useStore();

  const [gameState, setGameState] = useState<UnoGameState>({
    players: [],
    current_idx: 0,
    direction: 1,
    top_discard: null,
    chosen_color: null,
    pending_draw: 0,
    public_counts: [],
    hand: [],
    winner: null,
    gameStarted: false,
  });

  const [selectedCard, setSelectedCard] = useState<UnoCard | null>(null);
  const [showColorPicker, setShowColorPicker] = useState(false);
  const [pendingWildCard, setPendingWildCard] = useState<UnoCard | null>(null);

  useEffect(() => {
    ws.connect();

    // ✅ Set the game ID in the store when component mounts
    if (id) {
      setGameId(id);
      console.log("[Uno] Game ID set:", id);
    }

    const unsub = ws.onMessage((msg) => {
      console.log("[Uno] Received:", msg);

      if (msg.type === "Uno") {
        const data = msg.data;

        // Update public game state
        setGameState((prev) => {
          const newState = { ...prev };

          if (data.players) {
            newState.players = data.players;
            newState.gameStarted = data.players.length > 0;
          }
          if (data.current_idx !== null && data.current_idx !== undefined) {
            newState.current_idx = data.current_idx;
          }
          if (data.direction !== null && data.direction !== undefined) {
            newState.direction = data.direction;
          }
          if (data.top_discard !== undefined) {
            newState.top_discard = data.top_discard;
          }
          if (data.chosen_color !== undefined) {
            newState.chosen_color = data.chosen_color;
          }
          if (data.pending_draw !== null && data.pending_draw !== undefined) {
            newState.pending_draw = data.pending_draw;
          }
          if (data.public_counts) {
            newState.public_counts = data.public_counts;
          }
          if (data.winner !== undefined) {
            newState.winner = data.winner;
          }

          // Update private hand
          if (data.hand) {
            newState.hand = data.hand;
          }

          return newState;
        });
      }

      if (msg.type === "GameRoom") {
        const { players: serverPlayers } = msg.data;
        if (serverPlayers && Array.isArray(serverPlayers)) {
          setPlayers(serverPlayers);
        }
      }

      // ✅ Handle chat messages
      if (msg.type === "Chat") {
        console.log("[Uno] Chat message received:", msg.data);
        addChatMessage({
          player_name: msg.data.player_name,
          chat_message: msg.data.chat_message,
          time: msg.data.time,
        });
      }
    });

    return () => unsub();
  }, [setPlayers, setGameId, addChatMessage, id]); // ✅ Added dependencies

  const handleStartGame = () => {
    if (!id || !playerName) return;

    console.log("[Uno] Starting game");
    ws.send({
      type: "Uno",
      data: {
        game_id: id,
        player_name: playerName,
        action: "start",
      },
    });
  };

  const handlePlayCard = (card: UnoCard) => {
    if (!id || !playerName) return;

    // Check if this is a Wild or WildDrawFour - need to choose color
    if (card.rank === "Wild" || card.rank === "WildDrawFour") {
      setPendingWildCard(card);
      setShowColorPicker(true);
      return;
    }

    console.log(`[Uno] Playing card:`, card);
    ws.send({
      type: "Uno",
      data: {
        game_id: id,
        player_name: playerName,
        action: "play_card",
        card: card,
      },
    });

    setSelectedCard(null);
  };

  const handlePlayWildWithColor = (color: string) => {
    if (!pendingWildCard || !id || !playerName) return;

    console.log(`[Uno] Playing wild card with color:`, color);
    ws.send({
      type: "Uno",
      data: {
        game_id: id,
        player_name: playerName,
        action: "play_card",
        card: pendingWildCard,
        choose_color: color,
      },
    });

    setShowColorPicker(false);
    setPendingWildCard(null);
    setSelectedCard(null);
  };

  const handleDrawCard = () => {
    if (!id || !playerName) return;

    console.log("[Uno] Drawing card");
    ws.send({
      type: "Uno",
      data: {
        game_id: id,
        player_name: playerName,
        action: "draw_card",
      },
    });
  };

  const handlePassTurn = () => {
    if (!id || !playerName) return;

    console.log("[Uno] Passing turn");
    ws.send({
      type: "Uno",
      data: {
        game_id: id,
        player_name: playerName,
        action: "pass_turn",
      },
    });
  };

  const handleBackToRoom = () => {
    clearChatMessages();
    console.log("[Uno] Navigating back to match room");
    navigate(`/match/${id}`, { state: { fromBoard: true } });
  };

  const handleMainMenu = () => {
    console.log("[Uno] Leaving game and going to main menu");
    clearChatMessages();

    if (id && playerName) {
      ws.send({
        type: "GameRoom",
        data: {
          game: "uno",
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

    console.log("[Uno] State reset, navigating to dashboard");
    navigate("/");
  };

  const getCardColor = (card: UnoCard): string => {
    return COLOR_MAP[card.color] || "#6B7280";
  };

  const getCardDisplay = (card: UnoCard): string => {
    // Convert rank to display format
    const rankMap: { [key: string]: string } = {
      "0": "0", "1": "1", "2": "2", "3": "3", "4": "4",
      "5": "5", "6": "6", "7": "7", "8": "8", "9": "9",
      Skip: "🚫",
      Reverse: "🔄",
      DrawTwo: "+2",
      Wild: "🌈",
      WildDrawFour: "🌈+4",
    };
    return rankMap[card.rank] || card.rank;
  };

  const isMyTurn = () => {
    if (!gameState.gameStarted || gameState.players.length === 0) return false;
    const currentPlayer = gameState.players[gameState.current_idx];
    return currentPlayer === playerName;
  };

  const canPlayCard = (card: UnoCard): boolean => {
    if (!isMyTurn()) return false;
    if (!gameState.top_discard) return false;

    const top = gameState.top_discard;

    // Wild cards can always be played
    if (card.rank === "Wild" || card.rank === "WildDrawFour") return true;

    // Match color or rank
    return card.color === top.color || card.rank === top.rank;
  };

  const getCurrentPlayerDisplay = () => {
    if (!gameState.gameStarted || gameState.players.length === 0) return "Waiting...";
    return gameState.players[gameState.current_idx] || "Unknown";
  };

  return (
    <Grid mt="lg" gutter="xl" style={{ alignItems: "flex-start" }}>
      <Grid.Col span="auto">
        <Stack align="center">
          <Title order={1} style={{ fontSize: 40 }}>
            🃏 UNO Game 🎴
          </Title>
          <Text size="sm" c="dimmed">
            Game ID: <strong>{id}</strong>
          </Text>

          {/* Players Display */}
          <Group gap="lg" mt="md">
            {gameState.players.map((player, idx) => (
              <Badge
                key={idx}
                size="xl"
                variant={player === playerName ? "filled" : "light"}
                color={player === getCurrentPlayerDisplay() ? "green" : "gray"}
                style={{ padding: "12px 20px", fontSize: 16 }}
              >
                <Group gap="xs">
                  <Text>{player}</Text>
                  {player === playerName && <Text fw={700}>← YOU</Text>}
                  <Text c="dimmed">({gameState.public_counts[idx] || 0} cards)</Text>
                </Group>
              </Badge>
            ))}
          </Group>

          {/* Game not started */}
          {!gameState.gameStarted && (
            <Stack align="center" mt="xl" gap="md">
              <Alert color="yellow" style={{ width: "100%", maxWidth: 500 }}>
                <Text size="lg" fw={600} ta="center">
                  ⏳ Waiting for players... ({players.length} joined)
                </Text>
              </Alert>
              <Button size="lg" color="green" onClick={handleStartGame} disabled={players.length < 2}>
                🎮 Start Game
              </Button>
              <Text size="sm" c="dimmed">
                Need at least 2 players to start
              </Text>
            </Stack>
          )}

          {/* Game started */}
          {gameState.gameStarted && !gameState.winner && (
            <>
              {/* Current Turn */}
              <Alert
                color={isMyTurn() ? "blue" : "gray"}
                mt="md"
                style={{ width: "100%", maxWidth: 500 }}
              >
                <Group justify="center" gap="xs">
                  <Text size="lg" fw={600}>
                    {isMyTurn() ? "🎮 YOUR TURN!" : `⏳ ${getCurrentPlayerDisplay()}'s turn`}
                  </Text>
                  {gameState.pending_draw > 0 && (
                    <Badge color="red" size="lg">
                      +{gameState.pending_draw} cards penalty!
                    </Badge>
                  )}
                </Group>
              </Alert>

              {/* Discard Pile / Current Card */}
              <Paper
                shadow="xl"
                radius="md"
                p="xl"
                mt="xl"
                style={{
                  backgroundColor: "rgba(0, 0, 0, 0.5)",
                  width: "100%",
                  maxWidth: 400,
                }}
              >
                <Title order={3} ta="center" mb="md">
                  Current Card
                </Title>
                {gameState.top_discard ? (
                  <Paper
                    shadow="md"
                    radius="lg"
                    p="xl"
                    style={{
                      backgroundColor: getCardColor(gameState.top_discard),
                      color: "white",
                      textAlign: "center",
                      height: 180,
                      display: "flex",
                      flexDirection: "column",
                      justifyContent: "center",
                      alignItems: "center",
                      fontSize: "4rem",
                      fontWeight: "bold",
                      border: "4px solid rgba(255,255,255,0.3)",
                    }}
                  >
                    {getCardDisplay(gameState.top_discard)}
                    {gameState.chosen_color && (
                      <Text size="sm" mt="xs" style={{ fontSize: "1rem" }}>
                        Color: {gameState.chosen_color}
                      </Text>
                    )}
                  </Paper>
                ) : (
                  <Text ta="center" c="dimmed">
                    No card on pile
                  </Text>
                )}

                {/* Direction indicator */}
                <Text ta="center" mt="md" size="lg">
                  {gameState.direction === 1 ? "↻ Clockwise" : "↺ Counter-clockwise"}
                </Text>
              </Paper>

              {/* Action Buttons */}
              {isMyTurn() && (
                <Group mt="lg" justify="center">
                  <Button size="lg" color="blue" onClick={handleDrawCard}>
                    🃏 Draw Card
                  </Button>
                  <Button size="lg" color="orange" onClick={handlePassTurn}>
                    ⏭️ Pass Turn
                  </Button>
                </Group>
              )}

              {/* Player's Hand */}
              <Paper
                shadow="md"
                radius="md"
                p="md"
                mt="xl"
                style={{
                  backgroundColor: "rgba(0, 0, 0, 0.3)",
                  width: "100%",
                  maxWidth: 900,
                }}
              >
                <Title order={4} mb="md">
                  Your Hand ({gameState.hand.length} cards)
                </Title>
                <SimpleGrid cols={{ base: 4, sm: 5, md: 7 }} spacing="sm">
                  {gameState.hand.map((card, idx) => {
                    const playable = canPlayCard(card);
                    return (
                      <Paper
                        key={idx}
                        shadow="sm"
                        radius="md"
                        p="md"
                        style={{
                          backgroundColor: getCardColor(card),
                          color: "white",
                          textAlign: "center",
                          cursor: playable ? "pointer" : "not-allowed",
                          opacity: playable ? 1 : 0.5,
                          height: 120,
                          display: "flex",
                          flexDirection: "column",
                          justifyContent: "center",
                          fontSize: "2rem",
                          fontWeight: "bold",
                          border:
                            selectedCard === card
                              ? "3px solid #FFD700"
                              : "2px solid rgba(255,255,255,0.3)",
                          transition: "all 0.2s",
                        }}
                        onClick={() => {
                          if (playable) {
                            setSelectedCard(card);
                            handlePlayCard(card);
                          }
                        }}
                        onMouseEnter={(e) => {
                          if (playable) {
                            e.currentTarget.style.transform = "translateY(-10px)";
                            e.currentTarget.style.boxShadow = "0 8px 16px rgba(0,0,0,0.3)";
                          }
                        }}
                        onMouseLeave={(e) => {
                          e.currentTarget.style.transform = "translateY(0)";
                          e.currentTarget.style.boxShadow = "none";
                        }}
                      >
                        {getCardDisplay(card)}
                        <Text size="xs" mt="xs" style={{ fontSize: "0.7rem" }}>
                          {card.color}
                        </Text>
                      </Paper>
                    );
                  })}
                </SimpleGrid>
              </Paper>
            </>
          )}

          {/* Winner Display */}
          {gameState.winner && (
            <Stack align="center" mt="xl" gap="md">
              <Alert
                color={gameState.winner === playerName ? "green" : "blue"}
                title="🎉 GAME OVER!"
                style={{ width: "100%", maxWidth: 500, fontSize: 18 }}
              >
                <Text size="lg" fw={600}>
                  {gameState.winner === playerName
                    ? "🏆 YOU WON! Congratulations!"
                    : `${gameState.winner} won the game!`}
                </Text>
              </Alert>

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

          {/* Navigation buttons */}
          {gameState.gameStarted && !gameState.winner && (
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

      {/* Color Picker Modal for Wild Cards */}
      <Modal
        opened={showColorPicker}
        onClose={() => {
          setShowColorPicker(false);
          setPendingWildCard(null);
        }}
        title="Choose a Color"
        centered
      >
        <Stack gap="md">
          <Text ta="center" size="lg" fw={600}>
            Select the next color:
          </Text>
          <SimpleGrid cols={2} spacing="md">
            {["Red", "Yellow", "Green", "Blue"].map((color) => (
              <Button
                key={color}
                size="xl"
                style={{
                  backgroundColor: COLOR_MAP[color],
                  color: "white",
                  height: 100,
                  fontSize: "1.5rem",
                  fontWeight: "bold",
                }}
                onClick={() => handlePlayWildWithColor(color)}
              >
                {color}
              </Button>
            ))}
          </SimpleGrid>
        </Stack>
      </Modal>
    </Grid>
  );
}