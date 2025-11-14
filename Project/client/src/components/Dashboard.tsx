// client/src/components/Dashboard.tsx
import React from "react";
import { Image, Card, SimpleGrid, Title, Text } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { useStore } from "../state/store";
import { ws } from "../api/ws";

const games = [
  {
    name: "Tic Tac Toe",
    image: "https://upload.wikimedia.org/wikipedia/commons/3/32/Tic_tac_toe.svg",
    path: "/createjoin",
    gameType: "tictactoe" as const,
  },
  {
    name: "Uno",
    image: "https://cdn-icons-png.flaticon.com/512/1179/1179120.png",
    path: "/createjoin",
    gameType: "uno" as const,
  },
  {
    name: "Rock Paper Scissors",
    image: "https://cdn-icons-png.flaticon.com/512/1048/1048949.png",
    path: "/createjoin",
    gameType: "rockpaperscissors" as const,
  },
];

const Dashboard: React.FC = () => {
  const navigate = useNavigate();
  const {
    setGameId,
    setPlayerName,
    setPlayers,
    setBoard,
    setStatus,
    setWhosTurn,
    setGameType,
  } = useStore();

  const handleGameClick = (path: string, gameName: string, gameType: "tictactoe" | "rockpaperscissors" | "uno" | null) => {
    console.log(`[Dashboard] Starting ${gameName}`);

    // Clear all game state
    setGameId(null);
    setPlayerName("");
    setPlayers([]);
    setBoard([
      [0, 0, 0],
      [0, 0, 0],
      [0, 0, 0],
    ]);
    setStatus("waiting");
    setWhosTurn("");
    setGameType(gameType);

    // Clear sessionStorage
    sessionStorage.removeItem("ttt_playerName");

    // Close any existing WebSocket connection
    ws.close();

    console.log(`[Dashboard] State cleared, game type set to ${gameType}, navigating to CreateJoin`);

    // Navigate to the game
    navigate(path);
  };

  return (
    <div
      style={{
        minHeight: "40vh",
        paddingLeft: "40px",
        paddingRight: "40px",
        paddingTop: "10px",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        color: "white",
        marginTop: "1rem",
      }}
    >
      <div
        style={{
          margin: "10px",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          width: "100%",
        }}
      >
        <Title order={2} style={{ margin: "10px" }}>
          🎮 Welcome to Game Zone
        </Title>
      </div>

      <SimpleGrid
        cols={{ base: 1, sm: 2, md: 3 }}
        spacing="xl"
        style={{ maxWidth: "900px" }}
      >
        {games.map((game, index) => (
          <Card
            key={index}
            shadow="md"
            radius="lg"
            withBorder
            style={{
              cursor: game.gameType ? "pointer" : "not-allowed",
              backgroundColor: game.gameType
                ? "rgba(255,255,255,0.1)"
                : "rgba(100,100,100,0.1)",
              transition: "transform 0.25s ease, box-shadow 0.25s ease",
              opacity: game.gameType ? 1 : 0.5,
            }}
            onClick={() =>
              game.gameType && handleGameClick(game.path, game.name, game.gameType)
            }
            onMouseEnter={(e) => {
              if (game.gameType) {
                e.currentTarget.style.transform = "scale(1.05)";
                e.currentTarget.style.boxShadow = "0 8px 16px rgba(0,0,0,0.4)";
              }
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.transform = "scale(1)";
              e.currentTarget.style.boxShadow = "none";
            }}
          >
            <Image
              src={game.image}
              alt={game.name}
              height={120}
              fit="contain"
              radius="md"
              mb="sm"
            />
            <Text ta="center" fz="lg" fw={600}>
              {game.name}
            </Text>
            {!game.gameType && (
              <Text ta="center" fz="xs" c="dimmed" mt="xs">
                Coming Soon
              </Text>
            )}
          </Card>
        ))}
      </SimpleGrid>
    </div>
  );
};

export default Dashboard;