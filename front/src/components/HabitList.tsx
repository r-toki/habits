import {
  Box,
  Center,
  Flex,
  HStack,
  Icon,
  IconButton,
  Link,
  Menu,
  MenuButton,
  MenuDivider,
  MenuItem,
  MenuList,
  Spinner,
  Stack,
  Table,
  TableContainer,
  Tbody,
  Td,
  Tr,
} from '@chakra-ui/react';
import { GoKebabVertical } from 'react-icons/go';
import { TbCheck, TbQuestionMark, TbX } from 'react-icons/tb';
import { useNavigate } from 'react-router-dom';

import { useHabits } from '@/hooks/useHabits';

export const HabitList = () => {
  const navigate = useNavigate();

  const { habitsQuery, setHabitsQuery, habits, deleteHabit, archiveHabit, swapHabit } = useHabits();

  const onDeleteHabit = async (id: string) => {
    if (window.confirm('Delete?')) await deleteHabit.mutate(id);
  };
  const onArchiveHabit = async (id: string) => {
    if (window.confirm('Archive?')) await archiveHabit.mutate(id);
  };

  const onUpHabit = async (idx: number) => {
    const upper_id = habits.data![idx - 1].id;
    const lower_id = habits.data![idx].id;
    await swapHabit.mutate({ habitId1: upper_id, habitId2: lower_id });
  };
  const onDownHabit = async (idx: number) => {
    const upper_id = habits.data![idx].id;
    const lower_id = habits.data![idx + 1].id;
    await swapHabit.mutate({ habitId1: lower_id, habitId2: upper_id });
  };

  return (
    <Stack px="2">
      {habits.isLoading && (
        <Center>
          <Spinner />
        </Center>
      )}

      {habits.data?.length ? (
        <TableContainer>
          <Table>
            <Tbody>
              {habits.data.map((habit, idx) => (
                <Tr key={habit.id}>
                  <Td borderColor="transparent">
                    <Box>
                      <Menu placement="right-start">
                        <MenuButton color={habit.archived ? 'gray.400' : 'black'}>
                          {habit.name}
                        </MenuButton>
                        <MenuList>
                          <MenuItem onClick={() => navigate(`/habits/${habit.id}/edit`)}>
                            Edit
                          </MenuItem>

                          {habitsQuery.archived == 'false' && habits.data.length > 1 && (
                            <>
                              {idx != 0 && <MenuItem onClick={() => onUpHabit(idx)}>Up</MenuItem>}
                              {habits.data.length - 1 != idx && (
                                <MenuItem onClick={() => onDownHabit(idx)}>Down</MenuItem>
                              )}
                              <MenuDivider />
                            </>
                          )}

                          {!habit.archived && (
                            <MenuItem
                              onClick={() => onArchiveHabit(habit.id)}
                              disabled={archiveHabit.isLoading}
                            >
                              Archive
                            </MenuItem>
                          )}

                          {habit.archived && (
                            <MenuItem
                              onClick={() => onDeleteHabit(habit.id)}
                              disabled={deleteHabit.isLoading}
                            >
                              Delete
                            </MenuItem>
                          )}
                        </MenuList>
                      </Menu>
                    </Box>
                  </Td>
                  <Td borderColor="transparent">
                    {habitsQuery.archived == 'false' && (
                      <HStack spacing="1" justifyContent="end">
                        {habit.recentDoneList.map((b, idx) => {
                          const last = habit.recentDoneList.length == idx + 1;
                          const question = last && !b;
                          const check = b;
                          return (
                            <Box
                              key={idx}
                              display="flex"
                              alignItems="center"
                              justifyContent="center"
                              w="5"
                              h="5"
                              rounded="full"
                              color={question ? 'black' : check ? 'white' : 'black'}
                              bgColor={question ? 'gray.200' : check ? 'green.400' : 'gray.200'}
                            >
                              <Icon
                                as={question ? TbQuestionMark : check ? TbCheck : TbX}
                                fontSize="sm"
                              />
                            </Box>
                          );
                        })}
                      </HStack>
                    )}
                  </Td>
                </Tr>
              ))}
            </Tbody>
          </Table>
        </TableContainer>
      ) : (
        <Box alignSelf="center">there are no habits.</Box>
      )}

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
