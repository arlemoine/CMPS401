import { Button, Container, Title } from '@mantine/core';

function App() {
  return (
    <Container size="sm" style={{ paddingTop: 40 }}>
      <Title order={2} ta="center" mb="lg">
        Mantine + React + Vite is working!
      </Title>
      <Button fullWidth color="teal" radius="md" size="md">
        Click me
      </Button>
    </Container>
  );
}

export default App;