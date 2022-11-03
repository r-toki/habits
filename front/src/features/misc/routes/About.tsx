import { useEffect } from 'react';

import { AppLayout } from '@/components/AppLayout';

export const About = () => {
  useEffect(() => {
    fetch('http://127.0.0.1:8080').then(console.log);
  }, []);
  return (
    <AppLayout>
      <div>About</div>
    </AppLayout>
  );
};
