'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { r } from '@/utils';
import { Container } from './index.styled';

export default function SettingsPage() {
  const router = useRouter();

  useEffect(() => {
    router.push(r(`/system/settings/message`));
  }, []);

  return (
    <Container className="h-full">
      settings index page
    </Container>
  );
}
