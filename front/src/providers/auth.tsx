import { Center, Spinner } from '@chakra-ui/react';
import { useQuery } from '@tanstack/react-query';
import { createContext, ReactNode, useContext, useMemo } from 'react';

import { AuthUser, getAuthUser, getIndex as checkAuth } from '@/lib/auth';
import { getIndex as checkBackend } from '@/lib/backend';
import { assertDefined } from '@/utils/assert-defined';

type State = {
  initialized: boolean;
  authUser: AuthUser | undefined;
};

const useAuthProvider = (): State => {
  const { isInitialLoading: isCheckInitializing } = useQuery({
    queryKey: ['check'],
    queryFn: () => Promise.all([checkAuth(), checkBackend()]),
  });

  const { data: authUser, isInitialLoading: isMeInitializing } = useQuery({
    queryKey: ['authUser'],
    queryFn: getAuthUser,
    enabled: !isCheckInitializing,
    retry: false,
  });

  const initialized = useMemo(
    () => !isCheckInitializing && !isMeInitializing,
    [isCheckInitializing, isMeInitializing],
  );

  return {
    initialized,
    authUser,
  };
};

const AuthContext = createContext<State | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const state = useAuthProvider();
  if (!state.initialized)
    return (
      <Center h="full">
        <Spinner />
      </Center>
    );
  return <AuthContext.Provider value={state}>{children}</AuthContext.Provider>;
};

export const useAuth = () => {
  const state = useContext(AuthContext);
  assertDefined(state);
  return state;
};
