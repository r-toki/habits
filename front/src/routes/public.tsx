import { SignIn, SignUp } from '@/features/misc';

export const publicRoutes = [
  { path: '/sign-up', element: <SignUp /> },
  { path: '/sign-in', element: <SignIn /> },
];
