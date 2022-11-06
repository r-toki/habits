import { Box, Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { FormEventHandler } from 'react';

import { AppLink } from '@/components/AppLink';
import { AuthLayout } from '@/components/AuthLayout';
import { useTextInput } from '@/hooks/useTextInput';
import { createUserSession } from '@/lib/auth';
import { useAuth } from '@/providers/auth';

export const SignIn = () => {
  const { fetchUser } = useAuth();

  const nameInput = useTextInput();
  const passwordInput = useTextInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    await createUserSession({
      name: nameInput.value,
      password: passwordInput.value,
    });
    await fetchUser();
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

          <Button type="submit">Sign In</Button>
        </Stack>
      </form>

      <AppLink to="/sign-up">to sign up</AppLink>
    </AuthLayout>
  );
};
