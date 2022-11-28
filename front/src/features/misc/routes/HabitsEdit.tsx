import { Button, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react';
import { useMutation, useQuery } from '@tanstack/react-query';
import { FormEventHandler, useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import { AppLayout } from '@/components/AppLayout';
import { useAppToast } from '@/hooks/useAppToast';
import { useTextInput } from '@/hooks/useTextInput';
import { getHabit, updateHabit as updateHabitFn } from '@/lib/backend';

export const HabitsEdit = () => {
  const { habitId } = useParams();
  const navigate = useNavigate();
  const toast = useAppToast();

  const habit = useQuery({
    queryKey: ['habits', habitId],
    queryFn: () => getHabit(habitId!),
  });

  const updateHabit = useMutation({
    mutationFn: updateHabitFn,
    onSuccess: () => {
      toast({ status: 'success', title: 'Updated.' });
      navigate('/home');
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const nameInput = useTextInput();
  useEffect(() => {
    if (habit.data) {
      nameInput.set(habit.data.name);
    }
  }, [habit.data]);

  const onSubmit: FormEventHandler = async (e) => {
    e.preventDefault();

    await updateHabit.mutate({ habitId: habitId!, name: nameInput.value });
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
