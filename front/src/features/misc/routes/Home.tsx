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
import { deleteHabit as deleteHabitFn, getHabits } from '@/lib/backend';

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

  const onArchiveHabit = async (id: string) => {
    if (window.confirm('Archive?')) toast({ status: 'success', title: 'Archived.' });
  };

  return (
    <AppLayout>
      <Stack>
        {habits?.map((habit) => (
          <Flex key={habit.id} justifyContent="space-between" alignItems="center" px="4">
            <Box>{habit.name}</Box>
            <Box>
              <Menu placement="top-end">
                <MenuButton as={IconButton} icon={<GoKebabVertical />} size="xs" />
                <MenuList>
                  <MenuItem onClick={() => onArchiveHabit(habit.id)}>Archive</MenuItem>
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
