import { Box, Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { FormEventHandler } from 'react';

import { AppLink } from '@/components/AppLink';
import { AuthLayout } from '@/components/AuthLayout';
import { useAppToast } from '@/hooks/useAppToast';
import { useTextInput } from '@/hooks/useTextInput';
import { createAuthUserSession } from '@/lib/auth';

export const SignIn = () => {
  const toast = useAppToast();

  const client = useQueryClient();
  const mutation = useMutation({
    mutationFn: createAuthUserSession,
    onSuccess: () => {
      client.invalidateQueries(['authUser']);
      client.invalidateQueries(['me']);
      toast({ status: 'success', title: 'Signed in.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const nameInput = useTextInput();
  const passwordInput = useTextInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

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

          <Button type="submit" disabled={mutation.isLoading}>
            Sign In
          </Button>
        </Stack>
      </form>

      <AppLink to="/sign-up">to sign up</AppLink>
    </AuthLayout>
  );
};
