import { Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { FormEventHandler } from 'react';
import { Navigate, useNavigate, useParams } from 'react-router-dom';

import { AppLayout } from '@/components/AppLayout';
import { useAppToast } from '@/hooks/useAppToast';
import { useTextInput } from '@/hooks/useTextInput';
import { Habit, updateHabit as updateHabitFn } from '@/lib/backend';

export const HabitsEdit = () => {
  const client = useQueryClient();
  const { habitId } = useParams();

  const habit = client
    .getQueriesData<Habit>(['habits'])
    .flatMap(([, v]) => v)
    .find((v) => v?.id == habitId);

  if (!habit) return <Navigate to="/home" />;

  const navigate = useNavigate();
  const toast = useAppToast();

  const nameInput = useTextInput(habit.name);

  const updateHabit = useMutation({
    mutationFn: updateHabitFn,
    onSuccess: () => {
      toast({ status: 'success', title: 'Updated.' });
      navigate('/home');
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    await updateHabit.mutate({ habitId: habitId!, name: nameInput.value });
  };

  return (
    <AppLayout title="Edit Habit" back="/home">
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
