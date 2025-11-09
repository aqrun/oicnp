'use client';

import { redirect } from 'next/navigation';
import { useEffect } from 'react';
import { r } from '@/utils';

export default function SystemPage() {

  useEffect(() => {
    redirect(r('/system/users'));
  }, []);

  return (
    <>system</>
  );
}
