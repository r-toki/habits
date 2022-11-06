import { Navigate } from 'react-router-dom';

import { useAuth } from '@/providers/auth';

export const Landing = () => {
  const { user } = useAuth();
  return user ? <Navigate to="/app" /> : <Navigate to="/sign-in" />;
};
