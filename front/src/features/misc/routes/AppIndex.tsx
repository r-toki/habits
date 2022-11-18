import { Box } from '@chakra-ui/react';

import { AppLayout } from '@/components/AppLayout';
import { useAuth } from '@/providers/auth';

export const AppIndex = () => {
  const { authUser } = useAuth();
  return (
    <AppLayout>
      <Box>uid: {authUser!.uid}</Box>
    </AppLayout>
  );
};
