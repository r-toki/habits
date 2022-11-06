import { SignUp } from '@/features/misc';
import { SignIn } from '@/features/misc';

export const publicRoutes = [
  { path: '/sign-up', element: <SignUp /> },
  { path: '/sign-in', element: <SignIn /> },
];
