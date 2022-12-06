import { Navigate } from 'react-router-dom';

import { useAuth } from '@/providers/auth';
import { useMe } from '@/providers/me';

export const Landing = () => {
  const { authUser } = useAuth();
  const { me } = useMe();
  return authUser && me ? <Navigate to="/home" /> : <Navigate to="/sign-in" />;
};
