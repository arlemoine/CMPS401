// client/src/components/ChatBox.tsx
import { useState, useEffect, useRef } from "react";
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
import { MessageSquare, X, SendHorizonal } from "lucide-react";
import { useStore } from "../state/store";

export default function ChatBox() {
  const { playerName, gameId, chatMessages } = useStore();
  const [message, setMessage] = useState("");
  const [isOpen, setIsOpen] = useState(false);
  const viewport = useRef<HTMLDivElement>(null);

  // Debug: Log when chatMessages change
  useEffect(() => {
    console.log("[ChatBox] chatMessages updated:", chatMessages);
  }, [chatMessages]);

  // Debug: Log gameId and playerName
  useEffect(() => {
    console.log("[ChatBox] gameId:", gameId, "playerName:", playerName);
  }, [gameId, playerName]);

  // Auto-scroll to bottom when new messages appear
  useEffect(() => {
    viewport.current?.scrollTo({
      top: viewport.current.scrollHeight,
      behavior: "smooth",
    });
  }, [chatMessages]);

  const sendChat = () => {
    if (!message.trim() || !gameId) {
      console.warn("[ChatBox] Cannot send - message or gameId missing", { message, gameId });
      return;
    }
    
    console.log("[ChatBox] Sending chat message:", {
      game_id: gameId,
      player_name: playerName,
      chat_message: message,
    });

    // Send with action: "send" as per protocol
    ws.send({
      type: "Chat",
      data: {
        action: "send",
        game_id: gameId,
        player_name: playerName,
        chat_message: message,
        time: "", // Server will fill this in
      },
    });
    setMessage("");
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendChat();
    }
  };

  return (
    <>
      {/* Floating toggle button */}
      <ActionIcon
        pos="fixed"
        bottom={24}
        right={24}
        size="xl"
        radius="xl"
        variant="filled"
        color="blue"
        onClick={() => setIsOpen((o) => !o)}
        style={{
          zIndex: 2000,
          boxShadow: "0 6px 12px rgba(0,0,0,0.25)",
          transition: "transform 0.3s ease",
          transform: isOpen ? "rotate(90deg)" : "rotate(0deg)",
        }}
      >
        {isOpen ? <X /> : <MessageSquare />}
      </ActionIcon>

      {/* Slide chat panel */}
      <Transition mounted={isOpen} transition="slide-left" duration={400} timingFunction="ease">
        {(styles) => (
          <Paper
            shadow="xl"
            radius="lg"
            p="md"
            style={{
              ...styles,
              position: "fixed",
              right: 0,
              bottom: 0,
              zIndex: 1500,
              display: "flex",
              flexDirection: "column",
              width: 320,
              height: 500,
              background: "rgba(255, 255, 255, 0.25)",
              backdropFilter: "blur(12px)",
              border: "1px solid rgba(255,255,255,0.3)",
              borderTopLeftRadius: "1rem",
              borderTopRightRadius: "1rem",
              boxShadow: "0 -6px 24px rgba(0,0,0,0.3)",
            }}
          >
            {/* Header */}
            <Group justify="space-between" mb="xs">
              <Text fw={700} size="lg" color="blue">
                💬 Game Chat
              </Text>
              <ActionIcon color="dark" variant="light" onClick={() => setIsOpen(false)}>
                <X size={18} />
              </ActionIcon>
            </Group>

            {/* Messages */}
            <ScrollArea
              viewportRef={viewport}
              h={360}
              offsetScrollbars
              style={{
                border: "1px solid rgba(255,255,255,0.2)",
                borderRadius: "8px",
                padding: "8px",
                backgroundColor: "rgba(255,255,255,0.05)",
              }}
            >
              {chatMessages.length === 0 && (
                <Text ta="center" c="dimmed" mt="sm">
                  No messages yet. Start chatting! ✨
                </Text>
              )}

              {chatMessages.map((c, i) => {
                const isSelf = c.player_name === playerName;
                return (
                  <Box
                    key={i}
                    style={{
                      display: "flex",
                      justifyContent: isSelf ? "flex-end" : "flex-start",
                      marginBottom: 6,
                    }}
                  >
                    <Box
                      style={{
                        backgroundColor: isSelf ? "#228be6" : "#e9ecef",
                        color: isSelf ? "white" : "black",
                        padding: "8px 12px",
                        borderRadius: "12px",
                        maxWidth: "80%",
                        wordWrap: "break-word",
                        boxShadow: "0 2px 6px rgba(0,0,0,0.1)",
                      }}
                    >
                      <Text fw={600} size="sm">
                        {isSelf ? "You" : c.player_name}
                      </Text>
                      <Text size="sm">{c.chat_message}</Text>
                      <Text size="xs" c={isSelf ? "gray.1" : "gray.6"} mt={2}>
                        {c.time}
                      </Text>
                    </Box>
                  </Box>
                );
              })}
            </ScrollArea>

            {/* Input */}
            <Box mt="sm" style={{ display: "flex", gap: 8 }}>
              <TextInput
                placeholder="Type a message..."
                value={message}
                onChange={(e) => setMessage(e.currentTarget.value)}
                onKeyDown={handleKeyDown}
                style={{
                  flex: 1,
                  background: "rgba(255,255,255,0.3)",
                  borderRadius: "8px",
                }}
              />
              <ActionIcon
                onClick={sendChat}
                size="lg"
                radius="md"
                color="blue"
                variant="filled"
                disabled={!message.trim()}
              >
                <SendHorizonal size={18} />
              </ActionIcon>
            </Box>
          </Paper>
        )}
      </Transition>
    </>
  );
}