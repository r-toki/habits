import { Box, Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { FormEventHandler } from 'react';

import { AppLink } from '@/components/AppLink';
import { AuthLayout } from '@/components/AuthLayout';
import { useAppToast } from '@/hooks/useAppToast';
import { useTextInput } from '@/hooks/useTextInput';
import { createAuthUser, destroyAuthUser } from '@/lib/auth';
import { createUser } from '@/lib/backend';

export const SignUp = () => {
  const toast = useAppToast();

  const client = useQueryClient();
  const mutation = useMutation({
    mutationFn: async ({ name, password }: { name: string; password: string }) => {
      try {
        await createAuthUser({ name, password });
        await createUser({ displayName: name });
      } catch (e) {
        await destroyAuthUser();
        throw e;
      }
    },
    onSuccess: () => {
      client.invalidateQueries(['authUser', 'me']);
      toast({ status: 'success', title: 'Signed up.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const nameInput = useTextInput();
  const passwordInput = useTextInput();
  const passwordConfirmInput = useTextInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    if (passwordInput.value !== passwordConfirmInput.value) {
      toast({ status: 'error', title: 'Password and confirmation do not match.' });
      return;
    }

    await mutation.mutate({
      name: nameInput.value,
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
            <FormLabel>Name</FormLabel>
            <Input required {...nameInput.bind} />
          </FormControl>

          <FormControl>
            <FormLabel>Password</FormLabel>
            <Input type="password" required autoComplete="off" {...passwordInput.bind} />
          </FormControl>

          <FormControl>
            <FormLabel>Password Confirm</FormLabel>
            <Input type="password" required autoComplete="off" {...passwordConfirmInput.bind} />
          </FormControl>

          <Button type="submit" disabled={mutation.isLoading}>
            Sign Up
          </Button>
        </Stack>
      </form>

      <AppLink to="/sign-in">to sign in</AppLink>
    </AuthLayout>
  );
};
