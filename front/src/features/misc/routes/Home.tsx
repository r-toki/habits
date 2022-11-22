import { HStack, Link, Stack } from '@chakra-ui/react';
import { useLocalStorage } from 'react-use';

import { AppLayout } from '@/components/AppLayout';
import { HabitList } from '@/components/HabitList';
import { RecordEdit } from '@/components/RecordEdit';

export const Home = () => {
  const [tab, setTab] = useLocalStorage<'habits' | 'record'>('home_tab', 'habits');

  return (
    <AppLayout>
      <Stack>
        <HStack alignSelf="center">
          <Link onClick={() => setTab('habits')} color={tab != 'habits' ? 'gray.400' : undefined}>
            habits
          </Link>
          <Link onClick={() => setTab('record')} color={tab != 'record' ? 'gray.400' : undefined}>
            record
          </Link>
        </HStack>

        {tab == 'habits' && <HabitList />}
        {tab == 'record' && <RecordEdit />}
      </Stack>
    </AppLayout>
  );
};
