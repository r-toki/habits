import { Box } from '@chakra-ui/react';

import { AppLayout } from '@/components/AppLayout';
import { useAuth } from '@/providers/auth';

export const AppIndex = () => {
  const { user } = useAuth();
  return (
    <AppLayout>
      <Box>id: {user!.id}</Box>
      <Box>name: {user!.name}</Box>
    </AppLayout>
  );
};
