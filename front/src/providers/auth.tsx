import { Center, Spinner } from '@chakra-ui/react';
import { useQuery } from '@tanstack/react-query';
import { createContext, ReactNode, useContext } from 'react';

import { AuthUser, getAuthUser } from '@/lib/auth';

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
  return { initialized: !isInitialLoading, authUser };
};

const AuthContext = createContext<State | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const state = useAuthProvider();
  if (!state.initialized)
    return (
      <Center h="75vh">
        <Spinner />
      </Center>
    );
  return <AuthContext.Provider value={state}>{children}</AuthContext.Provider>;
};

export const useAuth = () => {
  const state = useContext(AuthContext);
  return state!;
};
