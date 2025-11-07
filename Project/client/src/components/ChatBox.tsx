// client/src/components/Chatbox.tsx
import { useState, useEffect} from "react";
import {
  Box,
  TextInput,
  Button,
  ScrollArea,
  Paper,
  Text,
  ActionIcon,
  Transition,
  Group,
} from "@mantine/core";
import { ws } from "../api/ws";
import { MessageSquare, X } from "lucide-react";
import { useStore } from "../state/store";

export default function ChatBox() {
   const { playerName, gameId, chatMessages, addChatMessage } = useStore();
  const [message, setMessage] = useState("");
  const [isOpen, setIsOpen] = useState(false);

  // ✅ Handle incoming chat messages matching backend structure
  useEffect(() => {
       ws.connect();
    const unsub = ws.onMessage((msg) => {
      if (msg.type === "Chat") {
        addChatMessage(msg.data);
      }
    });

    return () => unsub();
  }, [addChatMessage]);

    const sendChat = () => {
    if (!message.trim() || !gameId) return;
    ws.send({
      type: "Chat",
      data: {
        game_id: gameId,
        player_name: playerName,
        chat_message: message,
        time: "",
      },
    });
    setMessage("");
  };

  return (
    <>
      {/* Floating toggle button */}
      <ActionIcon
        pos="fixed"
        bottom={20}
        right={20}
        size="xl"
        radius="xl"
        variant="filled"
        color="blue"
        onClick={() => setIsOpen((o) => !o)}
        style={{
          zIndex: 2000,
          boxShadow: "0 4px 8px rgba(0,0,0,0.3)",
          transition: "transform 0.3s ease",
          transform: isOpen ? "rotate(90deg)" : "rotate(0deg)",
        }}
      >
        {isOpen ? <X /> : <MessageSquare />}
      </ActionIcon>

      {/* Slide & stretch chat panel */}
      <Transition mounted={isOpen} transition="slide-left" duration={400} timingFunction="ease">
        {(styles) => (
          <Paper
            shadow="xl"
            radius="lg"
            p="md"
            style={{
              ...styles,
              position: "fixed",
              right: isOpen ? 0 : "-350px",
              bottom: 0,
              zIndex: 1500,
              display: "flex",
              flexDirection: "column",
              backgroundColor: "#c07f25ff",
              borderTopLeftRadius: "1rem",
              borderTopRightRadius: "1rem",
              boxShadow: "0 -4px 20px rgba(0,0,0,0.2)",
              transition: "right 0.4s ease, transform 0.4s ease",
              transform: isOpen ? "translateY(0)" : "translateY(100%)",
              width: "300px",
              height: "480px",
              resize: "both",
              overflow: "auto",
              minWidth: "260px",
              minHeight: "280px",
              maxWidth: "600px",
              maxHeight: "600px",
            }}
          >
            {/* Header */}
            <Group justify="space-between" mb="xs">
              <Text fw={700} size="lg" color="blue">
                Game Chat
              </Text>
              <ActionIcon color="black" variant="light" onClick={() => setIsOpen(false)}>
                <X size={18} />
              </ActionIcon>
            </Group>

            {/* Message area */}
            <ScrollArea h={200} style={{ border: "1px solid #ccc", borderRadius: "8px", padding: "8px" }}>
        {chatMessages.map((c, i) => (
          <Text key={i}>
            <strong>{c.player_name}:</strong> {c.chat_message} <em>{c.time}</em>
          </Text>
        ))}
      </ScrollArea>

            {/* Input box */}
            <Box mt="xs" style={{ display: "flex", gap: 8 }}>
              <TextInput
                placeholder="Type a message..."
                value={message}
                onChange={(e) => setMessage(e.currentTarget.value)}
                style={{ flex: 1 }}
              />
              <Button onClick={sendChat} radius="md">
                Send
              </Button>
            </Box>
          </Paper>
        )}
      </Transition>
    </>
  );
}