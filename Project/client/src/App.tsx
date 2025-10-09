import { useEffect, useState } from "react";
import { Container, Title, Button } from "@mantine/core";
import { ws } from "./api/ws";

function App() {
    const [status, setStatus] = useState("disconnected");

    useEffect(() => {
        ws.connect();
        const offOpen = ws.onOpen(() => setStatus("open"));
        const offClose = ws.onClose(() => setStatus("disconnected"));
        const offMsg = ws.onMessage((msg) => {
            // update Zustand state based on msg
        });
        return () => {
            offOpen();
            offClose();
            offMsg();
        };
    }, []);

    return (
        <Container size="sm" style={{ paddingTop: 40 }}>
            <Title order={2} ta="center" mb="lg">
                Tic-Tac-Toe Prototype
            </Title>

            <Button
                fullWidth
                color="teal"
                radius="md"
                size="md"
                onClick={() => {
                    // Test sending join + create_match messages
                    ws.send({ type: "join", payload: { displayName: "Adam" } });
                    ws.send({ type: "create_match", payload: {} });
                }}
            >
                Connect & Create Match
            </Button>
        </Container>
    );
}

export default App;
