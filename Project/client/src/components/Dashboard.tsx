import React from "react";
// import { signOut } from "firebase/auth";
// import { auth } from "../firebase";
import { Image, Card, SimpleGrid, Title, Text } from "@mantine/core";
import { useNavigate } from "react-router-dom";

const games = [
  {
    name: "Tic Tac Toe",
    image: "https://upload.wikimedia.org/wikipedia/commons/3/32/Tic_tac_toe.svg",
    path: "/createjoin",
  },
  {
    name: "Snake Game",
    image: "https://cdn-icons-png.flaticon.com/512/1179/1179120.png",
    path: "/tic-tac-toe",
  },
  {
    name: "Memory Match",
    image: "https://cdn-icons-png.flaticon.com/512/1688/1688400.png",
    path: "/tic-tac-toe",
  },
  {
    name: "Flappy Bird",
    image: "https://cdn-icons-png.flaticon.com/512/743/743007.png",
    path: "/tic-tac-toe",
  },
  {
    name: "2048",
    image: "https://cdn-icons-png.flaticon.com/512/906/906175.png",
    path: "/tic-tac-toe",
  },
  {
    name: "Rock Paper Scissors",
    image: "https://cdn-icons-png.flaticon.com/512/1048/1048949.png",
    path: "/tic-tac-toe",
  },
];

const Dashboard: React.FC = () => {
  const navigate = useNavigate();

//   const handleLogout = async () => {
//     await signOut(auth);
//     navigate("/login");
//   };

  return (
    
      <div
      style={{
        minHeight: "80vh",
        paddingLeft: "40px",
        paddingRight:"40px",
        paddingTop:"10px",
        // background:
        //   "linear-gradient(135deg, rgba(30,30,60,1) 0%, rgba(50,50,80,1) 100%)",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        color: "white",
        marginTop: "1rem",
      }}
    >
        <div style={{margin:"10px", display:"flex", alignItems:"center", justifyContent:"space-between", width:"100%"}}>
        <Title order={2}  style={{margin:"10px"}}> 🎮 Welcome to Game Zone</Title>
      {/* <Text mt="sm">You are logged in as {auth.currentUser?.email}</Text> */}
      {/* <Button size="xs" onClick={handleLogout}>Logout</Button> */}
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
              cursor: "pointer",
              backgroundColor: "rgba(255,255,255,0.1)",
              transition: "transform 0.25s ease, box-shadow 0.25s ease",
            }}
            onClick={() => navigate(game.path)}
            onMouseEnter={(e) => {
              (e.currentTarget.style.transform = "scale(1.05)");
              (e.currentTarget.style.boxShadow = "0 8px 16px rgba(0,0,0,0.4)");
            }}
            onMouseLeave={(e) => {
              (e.currentTarget.style.transform = "scale(1)");
              (e.currentTarget.style.boxShadow = "none");
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
          </Card>
        ))}
      </SimpleGrid>
    </div>

  );
};

export default Dashboard;
