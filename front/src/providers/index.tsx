import { Center, ChakraProvider, Spinner } from '@chakra-ui/react';
import { QueryClient, QueryClientProvider, useQuery } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { ReactNode, useMemo } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { getIndex as checkAuth } from '@/lib/auth';
import { getIndex as checkBackend } from '@/lib/backend';

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
          <HealthCheck>
            <AuthProvider>
              <MeProvider>
                <>
                  {children}
                  <ReactQueryDevtools initialIsOpen={false} />
                </>
              </MeProvider>
            </AuthProvider>
          </HealthCheck>
        </QueryClientProvider>
      </ChakraProvider>
    </BrowserRouter>
  );
};

const HealthCheck = ({ children }: { children: ReactNode }) => {
  const { status } = useQuery({
    queryKey: ['healthCheck'],
    queryFn: () => Promise.all([checkAuth(), checkBackend()]),
  });

  switch (status) {
    case 'loading':
      return (
        <Center h="75vh">
          <Spinner />
        </Center>
      );
    case 'success':
      return <>{children}</>;
    case 'error':
      return <Center h="full">500</Center>;
  }
};
