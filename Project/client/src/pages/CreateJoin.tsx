import { useState, useEffect } from "react";
import { Button, Group, TextInput, Title, Stack, Alert } from "@mantine/core";
import { ws } from "../api/ws";
import { useStore } from "../state/store";
import { useNavigate } from "react-router-dom";

export default function CreateJoin() {
  const navigate = useNavigate();
  const { displayName, setDisplayName, matchId } = useStore();
  const [joinCode, setJoinCode] = useState("");
  const [error, setError] = useState("");

  // Navigate when matchId is set by server response
  useEffect(() => {
    if (matchId) {
      navigate(`/match/${matchId}`);
    }
  }, [matchId, navigate]);

  const onCreate = () => {
    if (!displayName.trim()) {
      setError("Please enter a display name");
      return;
    }
    setError("");
    ws.send({ type: "join", payload: { displayName } });
    ws.send({ type: "create_match", payload: {} });
    // navigation happens in useEffect when matchId is set
  };

  const onJoin = () => {
    if (!joinCode.trim() || !displayName.trim()) {
      setError("Please enter both display name and match code");
      return;
    }
    setError("");
    const matchIdUpper = joinCode.trim().toUpperCase();
    ws.send({ type: "join", payload: { displayName } });
    ws.send({ type: "join_match", payload: { matchId: matchIdUpper } });
    // navigation happens in useEffect when server confirms
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
      />

      <Group>
        <Button onClick={onCreate} color="teal">Create Match</Button>
      </Group>

      <TextInput
        label="Join by code"
        placeholder="e.g. ABCD"
        value={joinCode}
        onChange={(e) => setJoinCode(e.currentTarget.value)}
      />
      <Group>
        <Button onClick={onJoin}>Join Match</Button>
      </Group>
    </Stack>
  );
}