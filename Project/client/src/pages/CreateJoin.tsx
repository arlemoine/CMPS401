// client/src/pages/CreateJoin.tsx
import { useState, useEffect } from "react";
import { Button, Group, TextInput, Title, Stack, Alert } from "@mantine/core";
import { ws } from "../api/ws";
import { useStore } from "../state/store";
import { useNavigate } from "react-router-dom";

export default function CreateJoin() {
  const navigate = useNavigate();
  const { displayName, setDisplayName, matchId, setMatchId } = useStore();
  const [joinCode, setJoinCode] = useState("");
  const [error, setError] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [isJoining, setIsJoining] = useState(false);

  // Navigate when matchId is set by server response
  useEffect(() => {
    if (matchId) {
      navigate(`/match/${matchId}`);
    }
  }, [matchId, navigate]);

  // Reset loading states after timeout to prevent stuck buttons
   useEffect(() => {
    if (isCreating || isJoining) {
      const timeout = setTimeout(() => {
        setIsCreating(false);
        setIsJoining(false);
      }, 5000); // Reset after 5 seconds if no response
      
      return () => clearTimeout(timeout);
    }
  }, [isCreating, isJoining]);

  const onCreate = () => {
    if (!displayName.trim()) {
      setError("Please enter a display name");
      return;
    }
    setError("");
    setIsCreating(true);
    
    // Reset any previous match state
    setMatchId(null);
    
    ws.send({ type: "join", payload: { displayName: displayName.trim() } });
    ws.send({ type: "create_match", payload: {} });
  };

  const onJoin = () => {
    if (!joinCode.trim() || !displayName.trim()) {
      setError("Please enter both display name and match code");
      return;
    }
    setError("");
    setIsJoining(true);
    
    const matchIdUpper = joinCode.trim().toUpperCase();
    
    // Reset any previous match state
    setMatchId(null);
    
    ws.send({ type: "join", payload: { displayName: displayName.trim() } });
    ws.send({ type: "join_match", payload: { matchId: matchIdUpper } });
  };

  return (
    <Stack gap="md" mt="lg">
      <Title order={3}>Start or Join a Match</Title>

      {error && <Alert color="red">{error}</Alert>}

      <TextInput
        label="Display name"
        placeholder="Your name"
        value={displayName}
        onChange={(e) => setDisplayName(e.currentTarget.value)}
        required
      />

      <Group>
        <Button 
          onClick={onCreate} 
          color="teal"
          loading={isCreating}
        >
          Create Match
        </Button>
      </Group>

      <TextInput
        label="Join by code"
        placeholder="e.g. ABCD"
        value={joinCode}
        onChange={(e) => setJoinCode(e.currentTarget.value)}
        required
      />
      <Group>
        <Button 
          onClick={onJoin}
          loading={isJoining}
        >
          Join Match
        </Button>
      </Group>
    </Stack>
  );
}