import { useRoutes } from 'react-router-dom';

import { useAuth } from '@/providers/auth';

import { commonRoutes } from './common';
import { publicRoutes } from './public';

export const AppRoutes = () => {
  const { user } = useAuth();

  const routes = publicRoutes;

  const element = useRoutes([...routes, ...commonRoutes]);

  return <>{element}</>;
};
