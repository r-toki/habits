import { useQuery } from '@tanstack/react-query';
import { createContext, ReactNode, useContext } from 'react';

import { getUser, User } from '@/lib/backend';
import { assertDefined } from '@/utils/assert-defined';

type State = {
  initialized: boolean;
  me: User | undefined;
};

const useMeProvider = (): State => {
  const { data: me, isInitialLoading } = useQuery({
    queryKey: ['me'],
    queryFn: getUser,
    retry: false,
  });
  return { initialized: !isInitialLoading, me };
};

const MeContext = createContext<State | undefined>(undefined);

export const MeProvider = ({ children }: { children: ReactNode }) => {
  const state = useMeProvider();
  if (!state.initialized) return null;
  return <MeContext.Provider value={state}>{children}</MeContext.Provider>;
};

export const useMe = () => {
  const state = useContext(MeContext);
  assertDefined(state);
  return state;
};
