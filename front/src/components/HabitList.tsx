import {
  Box,
  Center,
  Flex,
  HStack,
  IconButton,
  Link,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  Spinner,
  Stack,
} from '@chakra-ui/react';
import { GoKebabVertical } from 'react-icons/go';

import { useHabits } from '@/hooks/useHabits';

export const HabitList = () => {
  const { habitsQuery, setHabitsQuery, habits, deleteHabit, archiveHabit } = useHabits();

  const onDeleteHabit = async (id: string) => {
    if (window.confirm('Delete?')) await deleteHabit.mutate(id);
  };
  const onArchiveHabit = async (id: string) => {
    if (window.confirm('Archive?')) await archiveHabit.mutate(id);
  };

  return (
    <Stack px="2">
      {habits.isLoading && (
        <Center>
          <Spinner />
        </Center>
      )}
      {habits.data?.map((habit) => (
        <Flex key={habit.id} justifyContent="space-between" alignItems="center">
          <Box color={habit.archivedAt ? 'gray.400' : 'black'}>{habit.name}</Box>
          <Box>
            <Menu placement="bottom-end">
              <MenuButton as={IconButton} icon={<GoKebabVertical />} size="xs" />
              <MenuList>
                {!habit.archivedAt && (
                  <MenuItem
                    onClick={() => onArchiveHabit(habit.id)}
                    disabled={archiveHabit.isLoading}
                  >
                    Archive
                  </MenuItem>
                )}

                <MenuItem onClick={() => onDeleteHabit(habit.id)} disabled={deleteHabit.isLoading}>
                  Delete
                </MenuItem>
              </MenuList>
            </Menu>
          </Box>
        </Flex>
      ))}

      <HStack alignSelf="end">
        <Link
          onClick={() => setHabitsQuery({ archived: 'false' })}
          color={habitsQuery.archived != 'false' ? 'gray.400' : undefined}
        >
          unarchived
        </Link>
        <Link
          onClick={() => setHabitsQuery({ archived: 'true' })}
          color={habitsQuery.archived != 'true' ? 'gray.400' : undefined}
        >
          archived
        </Link>
        <Link
          onClick={() => setHabitsQuery({ archived: 'null' })}
          color={habitsQuery.archived != 'null' ? 'gray.400' : undefined}
        >
          all
        </Link>
      </HStack>
    </Stack>
  );
};
