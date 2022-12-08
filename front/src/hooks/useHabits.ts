import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import {
  archiveHabit as archiveHabitFn,
  deleteHabit as deleteHabitFn,
  GetHabits,
  getHabits,
  swapHabits as swapHabitsFn,
} from '@/lib/backend';

import { useAppToast } from './useAppToast';

export const useHabits = () => {
  const toast = useAppToast();
  const client = useQueryClient();

  const [habitsQuery, setHabitsQuery] = useState<Pick<GetHabits, 'archived'>>({ archived: false });
  const habits = useQuery({
    queryKey: ['habits', habitsQuery],
    queryFn: () => getHabits({ tz: 'Asia/Tokyo', ...habitsQuery }),
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

  const swapHabits = useMutation({
    mutationFn: swapHabitsFn,
    onSuccess: () => {
      client.invalidateQueries(['habits']);
      toast({ status: 'success', title: 'Swapped.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  return { habitsQuery, setHabitsQuery, habits, deleteHabit, archiveHabit, swapHabits };
};
