import { Box, Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { FormEventHandler } from 'react';

import { AppLink } from '@/components/AppLink';
import { AuthLayout } from '@/components/AuthLayout';
import { useTextInput } from '@/hooks/useTextInput';
import { createAuthUser } from '@/lib/auth';
import { useAuth } from '@/providers/auth';

export const SignUp = () => {
  const { fetchAuthUser } = useAuth();

  const nameInput = useTextInput();
  const passwordInput = useTextInput();
  const passwordConfirmInput = useTextInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    await createAuthUser({
      name: nameInput.value,
      password: passwordInput.value,
    });
    await fetchAuthUser();
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

          <Button type="submit">Sign Up</Button>
        </Stack>
      </form>

      <AppLink to="/sign-in">to sign in</AppLink>
    </AuthLayout>
  );
};
