import { HamburgerIcon } from '@chakra-ui/icons';
import {
  Box,
  Container,
  IconButton,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  Stack,
} from '@chakra-ui/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ReactNode } from 'react';

import { useAppToast } from '@/hooks/useAppToast';
import { destroyAuthUserSession } from '@/lib/auth';

export const AppLayout = ({ children }: { children: ReactNode }) => {
  const toast = useAppToast();

  const client = useQueryClient();
  const mutation = useMutation({
    mutationFn: destroyAuthUserSession,
    onSuccess: () => {
      client.setQueriesData(['authUser'], null);
      client.setQueriesData(['me'], null);
      toast({ status: 'success', title: 'Signed out.' });
    },
  });

  const onSignOut = async () => {
    await mutation.mutate();
  };

  return (
    <Container maxW="md" py="2">
      <Stack>
        <Box alignSelf="end">
          <Menu placement="top-end">
            <MenuButton as={IconButton} aria-label="menu" icon={<HamburgerIcon />} />
            <MenuList>
              <MenuItem onClick={onSignOut} disabled={mutation.isLoading}>
                Sign Out
              </MenuItem>
            </MenuList>
          </Menu>
        </Box>

        <Box>{children}</Box>
      </Stack>
    </Container>
  );
};
