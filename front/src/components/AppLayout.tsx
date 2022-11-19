import {
  Box,
  Container,
  Flex,
  IconButton,
  Menu,
  MenuButton,
  MenuDivider,
  MenuItem,
  MenuList,
  Stack,
} from '@chakra-ui/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { ReactNode, useMemo } from 'react';
import { GoThreeBars } from 'react-icons/go';
import { useNavigate } from 'react-router-dom';

import { useAppToast } from '@/hooks/useAppToast';
import { destroyAuthUserSession } from '@/lib/auth';
import { QUOTES } from '@/lib/quotes';

export const AppLayout = ({
  title = 'My Habit',
  children,
}: {
  title?: string;
  children: ReactNode;
}) => {
  const navigate = useNavigate();
  const toast = useAppToast();

  const client = useQueryClient();
  const signOut = useMutation({
    mutationFn: destroyAuthUserSession,
    onSuccess: () => {
      client.setQueryData(['authUser'], null);
      client.setQueryData(['me'], null);
      toast({ status: 'success', title: 'Signed out.' });
    },
  });

  const onSignOut = async () => {
    await signOut.mutate();
  };

  const quote = useMemo(() => QUOTES[new Date().getTime() % QUOTES.length].split('\n'), []);

  return (
    <Container maxW="md" h="full" py="2">
      <Stack h="full" spacing="4">
        <Flex justifyContent="end" alignItems="center" position="relative" h="40px">
          <Box
            position="absolute"
            left="50%"
            transform="translateX(-50%)"
            fontWeight="bold"
            fontSize="xl"
          >
            {title}
          </Box>

          <Box>
            <Menu placement="bottom-end">
              <MenuButton as={IconButton} icon={<GoThreeBars />} />
              <MenuList>
                <MenuItem onClick={() => navigate('/home')}>Home</MenuItem>
                <MenuItem onClick={() => navigate('/habits/new')}>Add a Habit</MenuItem>
                <MenuDivider />
                <MenuItem onClick={onSignOut} disabled={signOut.isLoading}>
                  Sign Out
                </MenuItem>
              </MenuList>
            </Menu>
          </Box>
        </Flex>

        <Box>{children}</Box>

        <Box
          as="i"
          fontWeight="bold"
          fontSize="sm"
          color="gray"
          p="4"
          borderWidth="1px"
          borderRadius="md"
        >
          <Box>{quote[0]}</Box>
          <Box textAlign="end">{quote[1]}</Box>
        </Box>
      </Stack>
    </Container>
  );
};
