import React, { useState } from "react";
import { signInWithEmailAndPassword } from "firebase/auth";
import { auth } from "../firebase";
import { TextInput, PasswordInput, Button, Paper, Title, Text } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useNavigate, Link } from "react-router-dom";

const Login: React.FC = () => {
  const navigate = useNavigate();
  const [error, setError] = useState("");

  const form = useForm({
    initialValues: { email: "", password: "" },
  });

  const handleLogin = async () => {
    try {
      setError("");
      await signInWithEmailAndPassword(auth, form.values.email, form.values.password);
      navigate("/dashboard");
    } catch (err: any) {
      setError("Invalid email or password");
    }
  };

  return (
    <Paper radius="md" p="xl" withBorder maw={400} mx="auto" mt="xl">
      <Title order={2} ta="center" mb="lg">Login</Title>

      <form onSubmit={form.onSubmit(handleLogin)}>
        <TextInput label="Email" placeholder="you@example.com" {...form.getInputProps("email")} required />
        <PasswordInput mt="md" label="Password" placeholder="••••••" {...form.getInputProps("password")} required />

        {error && <Text c="red" mt="sm">{error}</Text>}

        <Button fullWidth mt="lg" type="submit">Login</Button>

        <Text mt="md" ta="center">
          Don’t have an account? <Link to="/signup">Sign Up</Link>
        </Text>
      </form>
    </Paper>
  );
};

export default Login;
