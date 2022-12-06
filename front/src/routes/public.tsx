import { SignIn } from '@/pages/SignIn';
import { SignUp } from '@/pages/SignUp';

export const publicRoutes = [
  { path: '/sign-up', element: <SignUp /> },
  { path: '/sign-in', element: <SignIn /> },
];
