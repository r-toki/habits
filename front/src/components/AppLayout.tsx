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

import { destroyAuthUserSession } from '@/lib/auth';

export const AppLayout = ({ children }: { children: ReactNode }) => {
  const queryClient = useQueryClient();
  const mutation = useMutation({
    mutationFn: destroyAuthUserSession,
    onSuccess: () => {
      queryClient.setQueryData(['authUser'], null);
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
