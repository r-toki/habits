import { Container } from '@chakra-ui/react';
import { ReactNode } from 'react';

export const AuthLayout = ({ children }: { children: ReactNode }) => {
  return (
    <Container maxW="md" py="10">
      {children}
    </Container>
  );
};
