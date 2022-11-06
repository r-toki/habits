import { Outlet } from 'react-router-dom';

import { AppIndex } from '@/features/misc';

const App = () => <Outlet />;

export const protectedRoutes = [
  {
    path: '/app',
    element: <App />,
    children: [{ path: '', element: <AppIndex /> }],
  },
];
