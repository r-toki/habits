import { useRoutes } from 'react-router-dom';

import { commonRoutes } from './common';
import { publicRoutes } from './public';

export const AppRoutes = () => {
  const routes = publicRoutes;

  const element = useRoutes([...routes, ...commonRoutes]);

  return <>{element}</>;
};
