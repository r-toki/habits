import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import {
  archiveHabit as archiveHabitFn,
  deleteHabit as deleteHabitFn,
  getHabits,
  HabitsQuery,
  swapHabit as swapHabitFn,
} from '@/lib/backend';

import { useAppToast } from './useAppToast';

export const useHabits = () => {
  const toast = useAppToast();
  const client = useQueryClient();

  const [habitsQuery, setHabitsQuery] = useState<HabitsQuery>({ archived: false });
  const habits = useQuery({
    queryKey: ['habits', habitsQuery],
    queryFn: () => getHabits(habitsQuery),
  });

  const deleteHabit = useMutation({
    mutationFn: deleteHabitFn,
    onSuccess: () => {
      client.invalidateQueries(['habits']);
      toast({ status: 'success', title: 'Deleted.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const archiveHabit = useMutation({
    mutationFn: archiveHabitFn,
    onSuccess: () => {
      client.invalidateQueries(['habits']);
      toast({ status: 'success', title: 'Archived.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const swapHabit = useMutation({
    mutationFn: swapHabitFn,
    onSuccess: () => {
      client.invalidateQueries(['habits']);
      toast({ status: 'success', title: 'Swapped.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  return { habitsQuery, setHabitsQuery, habits, deleteHabit, archiveHabit, swapHabit };
};
