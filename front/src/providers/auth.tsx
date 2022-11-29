import { Center, Spinner } from '@chakra-ui/react';
import { getAuth, getIdToken, onAuthStateChanged, User as AuthUser } from 'firebase/auth';
import { createContext, ReactNode, useContext, useEffect, useState } from 'react';

import { tokenStorage } from '@/lib/token-storage';

type State = {
  initialized: boolean;
  authUser: AuthUser | undefined;
};

const useAuthProvider = (): State => {
  const [initialized, setInitialized] = useState(false);
  const [authUser, setAuthUser] = useState<AuthUser>();
  useEffect(() => {
    onAuthStateChanged(getAuth(), async (v) => {
      if (v) {
        setAuthUser(v);
        const token = await getIdToken(v);
        tokenStorage.set('access_token', token);
      } else {
        setAuthUser(undefined);
      }
      if (!initialized) setInitialized(true);
    });
  }, []);
  return { initialized, authUser };
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
