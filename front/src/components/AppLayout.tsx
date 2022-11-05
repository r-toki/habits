import { Container } from '@chakra-ui/react';
import { ReactNode } from 'react';

export const AppLayout = ({ children }: { children: ReactNode }) => {
  return <Container>{children}</Container>;
};
