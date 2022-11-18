import { Box } from '@chakra-ui/react';

import { AppLayout } from '@/components/AppLayout';
import { useAuth } from '@/providers/auth';
import { useMe } from '@/providers/me';

export const Home = () => {
  const { authUser } = useAuth();
  const { me } = useMe();
  return (
    <AppLayout>
      <Box>Home</Box>
      <Box>authUser?.id: {authUser?.id}</Box>
      <Box>authUser?.name: {authUser?.name}</Box>
      <Box>me?.id: {me?.id}</Box>
      <Box>me?.displayName: {me?.displayName}</Box>
    </AppLayout>
  );
};
