import {
  Box,
  Flex,
  IconButton,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  Stack,
} from '@chakra-ui/react';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { GoKebabVertical } from 'react-icons/go';

import { AppLayout } from '@/components/AppLayout';
import { useAppToast } from '@/hooks/useAppToast';
import {
  archiveHabit as archiveHabitFn,
  deleteHabit as deleteHabitFn,
  getHabits,
  unarchiveHabit as unarchiveHabitFn,
} from '@/lib/backend';

export const Home = () => {
  const toast = useAppToast();

  const { data: habits } = useQuery({ queryKey: ['habits'], queryFn: getHabits });

  const client = useQueryClient();

  const deleteHabit = useMutation({
    mutationFn: deleteHabitFn,
    onSuccess: () => {
      client.invalidateQueries(['habits']);
      toast({ status: 'success', title: 'Deleted.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });
  const onDeleteHabit = async (id: string) => {
    if (window.confirm('Delete?')) await deleteHabit.mutate(id);
  };

  const archiveHabit = useMutation({
    mutationFn: archiveHabitFn,
    onSuccess: () => {
      client.invalidateQueries(['habits']);
      toast({ status: 'success', title: 'Archived.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });
  const onArchiveHabit = async (id: string) => {
    if (window.confirm('Archive?')) await archiveHabit.mutate(id);
  };

  const unarchiveHabit = useMutation({
    mutationFn: unarchiveHabitFn,
    onSuccess: () => {
      client.invalidateQueries(['habits']);
      toast({ status: 'success', title: 'Unarchived.' });
    },
    onError: () => toast({ status: 'error', title: 'Failed.' }),
  });
  const onUnarchiveHabit = async (id: string) => {
    if (window.confirm('Archive?')) await unarchiveHabit.mutate(id);
  };

  return (
    <AppLayout>
      <Stack>
        {habits?.map((habit) => (
          <Flex key={habit.id} justifyContent="space-between" alignItems="center" px="4">
            <Box color={habit.archivedAt ? 'gray.400' : 'black'}>{habit.name}</Box>
            <Box>
              <Menu placement="bottom-end">
                <MenuButton as={IconButton} icon={<GoKebabVertical />} size="xs" />
                <MenuList>
                  {habit.archivedAt ? (
                    <MenuItem
                      onClick={() => onUnarchiveHabit(habit.id)}
                      disabled={unarchiveHabit.isLoading}
                    >
                      Unarchive
                    </MenuItem>
                  ) : (
                    <MenuItem
                      onClick={() => onArchiveHabit(habit.id)}
                      disabled={archiveHabit.isLoading}
                    >
                      Archive
                    </MenuItem>
                  )}

                  <MenuItem
                    onClick={() => onDeleteHabit(habit.id)}
                    disabled={deleteHabit.isLoading}
                  >
                    Delete
                  </MenuItem>
                </MenuList>
              </Menu>
            </Box>
          </Flex>
        ))}
      </Stack>
    </AppLayout>
  );
};
