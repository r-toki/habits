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
import { ReactNode } from 'react';

import { destroyUserSession } from '@/lib/auth';
import { useAuth } from '@/providers/auth';

export const AppLayout = ({ children }: { children: ReactNode }) => {
  const { resetUser } = useAuth();

  const onSignOut = async () => {
    await destroyUserSession();
    resetUser();
  };

  return (
    <Container maxW="md" py="2">
      <Stack>
        <Box alignSelf="end">
          <Menu placement="top-end">
            <MenuButton as={IconButton} aria-label="menu" icon={<HamburgerIcon />} />
            <MenuList>
              <MenuItem onClick={onSignOut}>Sign Out</MenuItem>
            </MenuList>
          </Menu>
        </Box>

        <Box>{children}</Box>
      </Stack>
    </Container>
  );
};
