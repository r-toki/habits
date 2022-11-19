import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import {
  archiveHabit as archiveHabitFn,
  deleteHabit as deleteHabitFn,
  getHabits,
  Habit,
  HabitsQuery,
} from '@/lib/backend';
import { find } from '@/utils/find';

import { useAppToast } from './useAppToast';

export const useHabits = () => {
  const toast = useAppToast();
  const client = useQueryClient();

  const [habitsQuery, setHabitsQuery] = useState<HabitsQuery>({ archived: 'false' });
  const habits = useQuery({
    queryKey: ['habits', habitsQuery],
    queryFn: () => getHabits(habitsQuery),
  });

  const deleteHabit = useMutation({
    mutationFn: deleteHabitFn,
    onSuccess: (_, id) => {
      client.setQueriesData<Habit[]>(['habits'], (prev) => prev?.filter((h) => h.id != id));
      toast({ status: 'success', title: 'Deleted.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const archiveHabit = useMutation({
    mutationFn: archiveHabitFn,
    onSuccess: (_, id) => {
      const target = find(habits.data!, id);
      target.archivedAt = new Date().toISOString();
      client.setQueriesData<Habit[]>(['habits'], (prev) =>
        prev?.map((h) => (h.id == id ? target : h)),
      );
      client.setQueryData<Habit[]>(['habits', { archived: 'false' }], (prev) =>
        prev?.filter((h) => h.id != id),
      );
      client.setQueryData<Habit[]>(['habits', { archived: 'true' }], (prev) =>
        [...(prev ?? []), target].sort((a, b) => (a.createdAt > b.createdAt ? -1 : 0)),
      );
      toast({ status: 'success', title: 'Archived.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  return { habitsQuery, setHabitsQuery, habits, deleteHabit, archiveHabit };
};
