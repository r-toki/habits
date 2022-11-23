import { Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { useMutation } from '@tanstack/react-query';
import { FormEventHandler } from 'react';
import { useNavigate } from 'react-router-dom';

import { AppLayout } from '@/components/AppLayout';
import { useAppToast } from '@/hooks/useAppToast';
import { useTextInput } from '@/hooks/useTextInput';
import { createHabit as createHabitFn } from '@/lib/backend';

export const HabitsNew = () => {
  const navigate = useNavigate();
  const toast = useAppToast();

  const createHabit = useMutation({
    mutationFn: createHabitFn,
    onSuccess: () => {
      toast({ status: 'success', title: 'Created.' });
      navigate('/home');
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const nameInput = useTextInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    await createHabit.mutate({ name: nameInput.value });
  };

  return (
    <AppLayout title="New Habit">
      <Stack spacing="4">
        <form onSubmit={onSubmit}>
          <Stack>
            <FormControl>
              <FormLabel>Name</FormLabel>
              <Input required {...nameInput.bind} />
            </FormControl>

            <Button type="submit" disabled={createHabit.isLoading}>
              Save
            </Button>
          </Stack>
        </form>
      </Stack>
    </AppLayout>
  );
};
