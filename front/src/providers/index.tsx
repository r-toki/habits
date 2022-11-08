import { ChakraProvider } from '@chakra-ui/react';
import { ReactNode } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { AuthProvider } from './auth';

export const AppProvider = ({ children }: { children: ReactNode }) => {
  return (
    <BrowserRouter>
      <ChakraProvider>
        <AuthProvider>{children}</AuthProvider>
      </ChakraProvider>
    </BrowserRouter>
  );
};
