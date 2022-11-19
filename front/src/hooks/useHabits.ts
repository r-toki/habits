import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';

import {
  archiveHabit as archiveHabitFn,
  deleteHabit as deleteHabitFn,
  getHabits,
  Habit,
  HabitsQuery,
  unarchiveHabit as unarchiveHabitFn,
} from '@/lib/backend';

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
      client.setQueryData<Habit[]>(['habits'], (prev) => prev?.filter((h) => h.id != id));
      toast({ status: 'success', title: 'Deleted.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const archiveHabit = useMutation({
    mutationFn: archiveHabitFn,
    onSuccess: (_, id) => {
      client.setQueryData<Habit[]>(['habits'], (prev) =>
        prev?.map((h) => (h.id == id ? { ...h, archivedAt: new Date().toISOString() } : h)),
      );
      toast({ status: 'success', title: 'Archived.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  const unarchiveHabit = useMutation({
    mutationFn: unarchiveHabitFn,
    onSuccess: (_, id) => {
      client.setQueryData<Habit[]>(['habits'], (prev) =>
        prev?.map((h) => (h.id == id ? { ...h, archivedAt: null } : h)),
      );
      toast({ status: 'success', title: 'Unarchived.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });

  return { habitsQuery, setHabitsQuery, habits, deleteHabit, archiveHabit, unarchiveHabit };
};
