import { ChakraProvider } from '@chakra-ui/react';
import { ReactNode } from 'react';
import { BrowserRouter } from 'react-router-dom';

export const AppProvider = ({ children }: { children: ReactNode }) => {
  return (
    <BrowserRouter>
      <ChakraProvider>{children}</ChakraProvider>
    </BrowserRouter>
  );
};
