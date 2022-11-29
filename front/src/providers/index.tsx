import { ChakraProvider } from '@chakra-ui/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { ReactNode, useMemo } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { AuthProvider } from './auth';
import { MeProvider } from './me';

// NOTE: BrowserRouter < Chakra < Auth
//       QueryClient < Auth
//       Auth < Me
export const AppProvider = ({ children }: { children: ReactNode }) => {
  const client = useMemo(() => new QueryClient(), []);
  return (
    <BrowserRouter>
      <ChakraProvider>
        <QueryClientProvider client={client}>
          <AuthProvider>
            <MeProvider>
              <>
                {children}
                <ReactQueryDevtools initialIsOpen={false} />
              </>
            </MeProvider>
          </AuthProvider>
        </QueryClientProvider>
      </ChakraProvider>
    </BrowserRouter>
  );
};
