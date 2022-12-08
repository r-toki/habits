import { Box, Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { createUserWithEmailAndPassword, getAuth, signInWithEmailAndPassword } from 'firebase/auth';
import { FormEventHandler } from 'react';

import { AppLink } from '@/components/AppLink';
import { AuthLayout } from '@/components/AuthLayout';
import { useAppToast } from '@/hooks/useAppToast';
import { useTextInput } from '@/hooks/useTextInput';
import { createMe } from '@/lib/backend';

export const SignUp = () => {
  const toast = useAppToast();
  const client = useQueryClient();

  const signUp = useMutation({
    mutationFn: async ({ email, password }: { email: string; password: string }) => {
      try {
        await createUserWithEmailAndPassword(getAuth(), email, password);
        await createMe({ displayName: email.split('@')[0] });
      } catch {
        console.warn('could not sign_up or create_user');
        try {
          await signInWithEmailAndPassword(getAuth(), email, password);
          await createMe({ displayName: email.split('@')[0] });
        } catch {
          console.warn('could not sign_in or create_user');
        }
      }
    },
    onSuccess: () => {
      client.invalidateQueries(['me']);
      toast({ status: 'success', title: 'Signed up.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const emailInput = useTextInput();
  const passwordInput = useTextInput();
  const passwordConfirmInput = useTextInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    if (passwordInput.value !== passwordConfirmInput.value) {
      toast({ status: 'error', title: 'Password and confirmation do not match.' });
      return;
    }

    await signUp.mutate({
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
            <Input required {...emailInput.bind} />
          </FormControl>

          <FormControl>
            <FormLabel>Password</FormLabel>
            <Input type="password" required {...passwordInput.bind} />
          </FormControl>

          <FormControl>
            <FormLabel>Password Confirm</FormLabel>
            <Input type="password" required {...passwordConfirmInput.bind} />
          </FormControl>

          <Button type="submit" disabled={signUp.isLoading}>
            Sign Up
          </Button>
        </Stack>
      </form>

      <AppLink to="/sign-in">to sign in</AppLink>
    </AuthLayout>
  );
};
