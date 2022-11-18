import { Navigate, useRoutes } from 'react-router-dom';

import { useAuth } from '@/providers/auth';

import { commonRoutes } from './common';
import { protectedRoutes } from './protected';
import { publicRoutes } from './public';

export const AppRoutes = () => {
  const { authUser } = useAuth();

  const routes = authUser ? protectedRoutes : publicRoutes;

  const element = useRoutes([
    ...routes,
    ...commonRoutes,
    { path: '*', element: <Navigate to="/" /> },
  ]);

  return <>{element}</>;
};
