import { ChakraProvider } from '@chakra-ui/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { ReactNode, useMemo } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { AuthProvider } from './auth';

// NOTE: BrowserRouter < ChakraProvider < AuthProvider
//       QueryClientProvider < AuthProvider

export const AppProvider = ({ children }: { children: ReactNode }) => {
  const client = useMemo(() => new QueryClient(), []);
  return (
    <BrowserRouter>
      <ChakraProvider>
        <QueryClientProvider client={client}>
          <AuthProvider>
            <>
              {children}
              <ReactQueryDevtools initialIsOpen={false} />
            </>
          </AuthProvider>
        </QueryClientProvider>
      </ChakraProvider>
    </BrowserRouter>
  );
};
