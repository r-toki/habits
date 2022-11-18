import { Navigate, useRoutes } from 'react-router-dom';

import { useAuth } from '@/providers/auth';
import { useMe } from '@/providers/me';

import { commonRoutes } from './common';
import { protectedRoutes } from './protected';
import { publicRoutes } from './public';

export const AppRoutes = () => {
  const { authUser } = useAuth();
  const { me } = useMe();
  const routes = authUser && me ? protectedRoutes : publicRoutes;
  const element = useRoutes([
    ...routes,
    ...commonRoutes,
    { path: '*', element: <Navigate to="/" /> },
  ]);
  return <>{element}</>;
};
