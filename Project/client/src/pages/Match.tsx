import { useEffect } from "react";
import { useParams } from "react-router-dom";
import { Stack, Title, Text, Badge, Group } from "@mantine/core";
import { useStore } from "../state/store";

export default function Match() {
  const { id } = useParams<{ id: string }>();
  const { matchId, setMatchId, players, matchStatus, me } = useStore();

  useEffect(() => {
    if (id && id !== matchId) setMatchId(id);
  }, [id, matchId, setMatchId]);

  return (
    <Stack gap="md" mt="lg">
      <Title order={3}>Match: {matchId ?? id}</Title>

      <Group>
        <Text>Status:</Text>
        <Badge color={matchStatus === "IN_PROGRESS" ? "green" : "yellow"}>
          {matchStatus}
        </Badge>
      </Group>

      {me && (
        <Text>
          You are: <strong>{me.displayName}</strong> playing as{" "}
          <strong>{me.mark}</strong>
        </Text>
      )}

      <Stack gap="xs">
        <Text fw={600}>Players ({players.length}/2):</Text>
        {players.map((p) => (
          <Group key={p.id}>
            <Badge>{p.mark}</Badge>
            <Text>{p.displayName}</Text>
          </Group>
        ))}
        {players.length < 2 && (
          <Text c="dimmed">Waiting for another player...</Text>
        )}
      </Stack>

      {matchStatus === "IN_PROGRESS" && (
        <Text c="blue">Game ready! (Board UI coming soon)</Text>
      )}
    </Stack>
  );
}