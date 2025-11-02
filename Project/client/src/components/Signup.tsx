import React, { useState } from "react";
import { createUserWithEmailAndPassword } from "firebase/auth";
import { auth } from "../firebase";
import { TextInput, PasswordInput, Button, Paper, Title, Text } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useNavigate, Link } from "react-router-dom";

const Signup: React.FC = () => {
  const navigate = useNavigate();
  const [error, setError] = useState("");

  const form = useForm({
    initialValues: { email: "", password: "" },
    validate: {
      email: (val) => (/^\S+@\S+$/.test(val) ? null : "Invalid email"),
      password: (val) => (val.length < 6 ? "Password must be at least 6 characters" : null),
    },
  });

  const handleSignup = async () => {
    try {
      setError("");
      await createUserWithEmailAndPassword(auth, form.values.email, form.values.password);
      navigate("/dashboard");
    } catch (err: any) {
      setError(err.message);
    }
  };

  return (
    <Paper radius="md" p="xl" withBorder maw={400} mx="auto" mt="xl">
      <Title order={2} ta="center" mb="lg">Create Account</Title>

      <form onSubmit={form.onSubmit(handleSignup)}>
        <TextInput label="Email" placeholder="you@example.com" {...form.getInputProps("email")} required />
        <PasswordInput mt="md" label="Password" placeholder="••••••" {...form.getInputProps("password")} required />

        {error && <Text c="red" mt="sm">{error}</Text>}

        <Button fullWidth mt="lg" type="submit">Sign Up</Button>

        <Text mt="md" ta="center">
          Already have an account? <Link to="/login">Log In</Link>
        </Text>
      </form>
    </Paper>
  );
};

export default Signup;
