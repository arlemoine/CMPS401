import { useEffect } from "react";
import { useParams } from "react-router-dom";
import { Title, Text } from "@mantine/core";
import { useStore } from "../state/store";

export default function Match() {
  const { id } = useParams(); // match id from url
  const { matchId, setMatchId } = useStore();

  useEffect(() => {
    if (id && id !== matchId) setMatchId(id);
  }, [id, matchId, setMatchId]);

  return (
    <>
      <Title order={3}>Match {matchId ?? id}</Title>
      <Text c="dimmed">Board and chat go here (coming next).</Text>
    </>
  );
}