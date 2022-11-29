import { Box, Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { getAuth, signInWithEmailAndPassword } from 'firebase/auth';
import { FormEventHandler } from 'react';

import { AppLink } from '@/components/AppLink';
import { AuthLayout } from '@/components/AuthLayout';
import { useAppToast } from '@/hooks/useAppToast';
import { useTextInput } from '@/hooks/useTextInput';

export const SignIn = () => {
  const toast = useAppToast();

  const client = useQueryClient();
  const signIn = useMutation({
    mutationFn: ({ email, password }: { email: string; password: string }) =>
      signInWithEmailAndPassword(getAuth(), email, password),
    onSuccess: () => {
      client.invalidateQueries(['authUser']);
      client.invalidateQueries(['me']);
      toast({ status: 'success', title: 'Signed in.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const emailInput = useTextInput();
  const passwordInput = useTextInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    await signIn.mutate({
      email: emailInput.value,
      password: passwordInput.value,
    });
  };

  return (
    <AuthLayout>
      <Box fontWeight="bold" fontSize="xl" textAlign="center">
        My Habit
      </Box>

      <form onSubmit={onSubmit}>
        <Stack>
          <FormControl>
            <FormLabel>Email</FormLabel>
            <Input type="email" required {...emailInput.bind} />
          </FormControl>

          <FormControl>
            <FormLabel>Password</FormLabel>
            <Input type="password" required {...passwordInput.bind} />
          </FormControl>

          <Button type="submit" disabled={signIn.isLoading}>
            Sign In
          </Button>
        </Stack>
      </form>

      <AppLink to="/sign-up">to sign up</AppLink>
    </AuthLayout>
  );
};
