import { useQuery } from '@tanstack/react-query';
import { createContext, ReactNode, useContext, useMemo } from 'react';

import { AuthUser, getAuthUser } from '@/lib/auth';
import { assertDefined } from '@/utils/assert-defined';

type State = {
  initialized: boolean;
  authUser: AuthUser | undefined;
};

const useAuthProvider = (): State => {
  const { data: authUser, isInitialLoading } = useQuery({
    queryKey: ['authUser'],
    queryFn: getAuthUser,
    retry: false,
  });
  const initialized = useMemo(() => !isInitialLoading, [isInitialLoading]);
  return { initialized, authUser };
};

const AuthContext = createContext<State | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const state = useAuthProvider();
  if (!state.initialized) return null;
  return <AuthContext.Provider value={state}>{children}</AuthContext.Provider>;
};

export const useAuth = () => {
  const state = useContext(AuthContext);
  assertDefined(state);
  return state;
};
