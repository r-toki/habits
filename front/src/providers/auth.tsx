import { Center, Spinner } from '@chakra-ui/react';
import { QueryObserverBaseResult, useQuery, useQueryClient } from '@tanstack/react-query';
import { createContext, ReactNode, useContext, useMemo } from 'react';

import { getIndex as checkAuth } from '@/lib/auth';
import { getIndex as checkBackend, getUser } from '@/lib/backend';
import { User } from '@/lib/backend/type';
import { assertDefined } from '@/utils/assert-defined';

type State = {
  initialized: boolean;
  user: User | undefined;
  fetchUser: QueryObserverBaseResult['refetch'];
  resetUser: () => void;
};

const useAuthProvider = (): State => {
  const { isInitialLoading: isCheckInitializing } = useQuery({
    queryKey: ['check'],
    queryFn: () => Promise.all([checkAuth(), checkBackend()]),
  });

  const {
    data: user,
    refetch: fetchUser,
    isInitialLoading: isMeInitializing,
  } = useQuery({
    queryKey: ['me'],
    queryFn: () => getUser(),
    enabled: !isCheckInitializing,
    retry: false,
  });

  const initialized = useMemo(
    () => !isCheckInitializing && !isMeInitializing,
    [isCheckInitializing, isMeInitializing],
  );

  const client = useQueryClient();
  const resetUser = () => client.setQueryData(['me'], null);

  return {
    initialized,
    user,
    fetchUser,
    resetUser,
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
