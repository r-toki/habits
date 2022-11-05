import { useEffect, useState } from 'react';

import { AppLayout } from '@/components/AppLayout';

export const About = () => {
  const [msg, setMsg] = useState('');
  useEffect(() => {
    fetch(import.meta.env.VITE_APP_AUTH_URL)
      .then((res) => res.text())
      .then(setMsg);
  }, []);
  return (
    <AppLayout>
      <div>About</div>
      <div>{msg}</div>
    </AppLayout>
  );
};
