import { HabitsEdit } from '@/pages/HabitsEdit';
import { HabitsNew } from '@/pages/HabitsNew';
import { Home } from '@/pages/Home';

export const protectedRoutes = [
  { path: '/home', element: <Home /> },
  { path: '/habits/new', element: <HabitsNew /> },
  { path: '/habits/:habitId/edit', element: <HabitsEdit /> },
];
