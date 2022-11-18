import { Navigate } from 'react-router-dom';

import { useAuth } from '@/providers/auth';

export const Landing = () => {
  const { authUser } = useAuth();
  return authUser ? <Navigate to="/app" /> : <Navigate to="/sign-in" />;
};
