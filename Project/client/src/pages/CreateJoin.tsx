import { useState } from "react";
import { Button, Group, TextInput, Title, Stack } from "@mantine/core";
import { ws } from "../api/ws";
import { useStore } from "../state/store";
import { useNavigate } from "react-router-dom";

export default function CreateJoin() {
  const navigate = useNavigate();
  const { displayName, setDisplayName, setMatchId } = useStore();
  const [joinCode, setJoinCode] = useState("");

  const onCreate = () => {
    ws.send({ type: "join", payload: { displayName } });
    ws.send({ type: "create_match", payload: {} });
    // navigation actually happens when we receive match_created in App.tsx listener
  };

  const onJoin = () => {
    // we'll wire join_match soon; for now redirect using the code
    if (joinCode.trim()) {
      setMatchId(joinCode.trim().toUpperCase());
      navigate(`/match/${joinCode.trim().toUpperCase()}`);
    }
  };

  return (
    <Stack gap="md" mt="lg">
      <Title order={3}>Start or Join a Match</Title>

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