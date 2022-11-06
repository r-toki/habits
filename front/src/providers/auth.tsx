import { createContext, ReactNode, useContext, useEffect, useState } from 'react';

import { getUser } from '@/lib/backend/api';
import { User } from '@/lib/backend/type';
import { assertDefined } from '@/utils/assert-defined';

type State = {
  initialized: boolean;
  user: User | undefined;
  fetchUser: () => Promise<void>;
  resetUser: () => void;
};

const useAuthProvider = (): State => {
  const [initialized, setInitialized] = useState(false);
  const [user, setUser] = useState<User>();

  const fetchUser = async () => {
    const user = await getUser();
    setUser(user);
  };

  const resetUser = () => setUser(undefined);

  useEffect(() => {
    (async () => {
      try {
        await fetchUser();
      } catch {
        console.log('[my habit] Unauthorized');
      } finally {
        setInitialized(true);
      }
    })();
  }, []);

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
  if (!state.initialized) return null;
  return <AuthContext.Provider value={state}>{children}</AuthContext.Provider>;
};

export const useAuth = () => {
  const state = useContext(AuthContext);
  assertDefined(state);
  return state;
};
