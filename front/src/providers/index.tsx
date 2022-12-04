import { ChakraProvider, extendTheme, theme } from '@chakra-ui/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { ReactNode, useMemo } from 'react';
import { BrowserRouter } from 'react-router-dom';

import { AuthProvider } from './auth';
import { MeProvider } from './me';

const extendedTheme = extendTheme({
  colors: {
    joy: {
      500: theme.colors.yellow[400],
      600: theme.colors.yellow[500],
    },
    trust: {
      500: theme.colors.green[400],
      600: theme.colors.green[500],
    },
    fear: {
      500: theme.colors.green[600],
      600: theme.colors.green[700],
    },
    surprise: {
      500: theme.colors.blue[400],
      600: theme.colors.blue[500],
    },
    sadness: {
      500: theme.colors.blue[600],
      600: theme.colors.blue[700],
    },
    disgust: {
      500: theme.colors.purple[500],
      600: theme.colors.purple[600],
    },
    anger: {
      500: theme.colors.red[400],
      600: theme.colors.red[500],
    },
    anticipation: {
      500: theme.colors.orange[400],
      600: theme.colors.orange[500],
    },
  },
});

export const AppProvider = ({ children }: { children: ReactNode }) => {
  const client = useMemo(() => new QueryClient(), []);
  return (
    <BrowserRouter>
      <ChakraProvider theme={extendedTheme}>
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
