import { Box, Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { FormEventHandler } from 'react';

import { AppLink } from '@/components/AppLink';
import { AuthLayout } from '@/components/AuthLayout';
import { useTextInput } from '@/hooks/useTextInput';
import { createUserSession } from '@/lib/auth';

export const SignIn = () => {
  const nameInput = useTextInput();
  const passwordInput = useTextInput();

  const onSubmit: FormEventHandler = (e) => {
    e.preventDefault();

    createUserSession({
      name: nameInput.value,
      password: passwordInput.value,
    }).then(console.log);
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
            <Input type="password" required {...passwordInput.bind} />
          </FormControl>

          <Button type="submit">Sign In</Button>
        </Stack>
      </form>

      <AppLink to="/sign-up">to sign up</AppLink>
    </AuthLayout>
  );
};
