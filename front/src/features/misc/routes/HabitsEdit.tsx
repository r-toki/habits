import { Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { useMutation } from '@tanstack/react-query';
import { FormEventHandler } from 'react';
import { useNavigate } from 'react-router-dom';

import { AppLayout } from '@/components/AppLayout';
import { useAppToast } from '@/hooks/useAppToast';
import { useTextInput } from '@/hooks/useTextInput';
import { updateHabit as updateHabitFn } from '@/lib/backend';

export const HabitsEdit = () => {
  const navigate = useNavigate();
  const toast = useAppToast();

  const updateHabit = useMutation({
    mutationFn: updateHabitFn,
    onSuccess: () => {
      toast({ status: 'success', title: 'Updated.' });
      navigate('/home');
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const nameInput = useTextInput();

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    await updateHabit.mutate({ habitId: '', name: nameInput.value });
  };

  return (
    <AppLayout title="Edit Habit">
      <Stack spacing="4">
        <form onSubmit={onSubmit}>
          <Stack>
            <FormControl>
              <FormLabel>Name</FormLabel>
              <Input required {...nameInput.bind} />
            </FormControl>

            <Button type="submit" disabled={updateHabit.isLoading}>
              Save
            </Button>
          </Stack>
        </form>
      </Stack>
    </AppLayout>
  );
};
