import { HStack, Link, Stack } from '@chakra-ui/react';
import { useState } from 'react';

import { AppLayout } from '@/components/AppLayout';
import { HabitsList } from '@/components/HabitsList';
import { RecordsEdit } from '@/components/RecordsEdit';

export const Home = () => {
  const [tab, setTab] = useState<'habits' | 'record'>('habits');

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

        {tab == 'habits' && <HabitsList />}
        {tab == 'record' && <RecordsEdit />}
      </Stack>
    </AppLayout>
  );
};
