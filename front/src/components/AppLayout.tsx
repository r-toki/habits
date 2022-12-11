import {
  Box,
  Container,
  Flex,
  Icon,
  IconButton,
  Menu,
  MenuButton,
  MenuDivider,
  MenuItem,
  MenuList,
  Stack,
} from '@chakra-ui/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { getAuth, signOut as signOutFn } from 'firebase/auth';
import { ReactNode, useMemo } from 'react';
import { FaArrowLeft } from 'react-icons/fa';
import { GoThreeBars } from 'react-icons/go';
import { useNavigate } from 'react-router-dom';

import { useAppToast } from '@/hooks/useAppToast';
import { QUOTES } from '@/lib/quotes';
import { useMe } from '@/providers/me';

export const AppLayout = ({
  title = 'My Habit',
  back,
  children,
}: {
  title?: string;
  back?: string;
  children: ReactNode;
}) => {
  const client = useQueryClient();
  const navigate = useNavigate();
  const toast = useAppToast();

  const { me } = useMe();

  const signOut = useMutation({
    mutationFn: () => signOutFn(getAuth()),
    onSuccess: () => {
      client.setQueryData(['me'], null);
      toast({ status: 'success', title: 'Signed out.' });
    },
  });

  const quote = useMemo(() => QUOTES[new Date().getTime() % QUOTES.length].split('\n'), []);

  return (
    <Container maxW="lg" py="2">
      <Stack spacing="4">
        <Flex justify="end" align="center" position="relative" h="40px">
          {back && (
            <Flex
              justify="center"
              align="center"
              position="absolute"
              left="0"
              w="10"
              h="10"
              cursor="pointer"
              onClick={() => navigate(back)}
            >
              <Icon as={FaArrowLeft} />
            </Flex>
          )}

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
                <MenuItem>{me!.displayName}</MenuItem>
                <MenuItem onClick={() => navigate('/habits/new')}>Add a Habit</MenuItem>
                <MenuDivider />
                <MenuItem onClick={() => signOut.mutate()} disabled={signOut.isLoading}>
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
