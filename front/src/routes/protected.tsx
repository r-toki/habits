import { Home } from '@/features/misc';
import { HabitsNew } from '@/features/misc/routes/HabitsNew';

export const protectedRoutes = [
  { path: '/home', element: <Home /> },
  { path: '/habits/new', element: <HabitsNew /> },
];
