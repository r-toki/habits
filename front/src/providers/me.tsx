import { Center, Spinner } from '@chakra-ui/react';
import { useQuery } from '@tanstack/react-query';
import { createContext, ReactNode, useContext } from 'react';

import { getMe, Me } from '@/lib/backend';

type State = {
  initialized: boolean;
  me: Me | undefined;
};

const useMeProvider = (): State => {
  const { data: me, isInitialLoading } = useQuery({
    queryKey: ['me'],
    queryFn: getMe,
    retry: false,
  });
  return { initialized: !isInitialLoading, me };
};

const MeContext = createContext<State | undefined>(undefined);

export const MeProvider = ({ children }: { children: ReactNode }) => {
  const state = useMeProvider();
  if (!state.initialized)
    return (
      <Center h="75vh">
        <Spinner />
      </Center>
    );
  return <MeContext.Provider value={state}>{children}</MeContext.Provider>;
};

export const useMe = () => {
  const state = useContext(MeContext);
  return state!;
};
