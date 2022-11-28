import { HabitsEdit, HabitsNew, Home } from '@/features/misc';

export const protectedRoutes = [
  { path: '/home', element: <Home /> },
  { path: '/habits/new', element: <HabitsNew /> },
  { path: '/habits/:habitId/edit', element: <HabitsEdit /> },
];
