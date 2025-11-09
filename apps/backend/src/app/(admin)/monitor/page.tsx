'use client';

import { redirect } from 'next/navigation';
import { useEffect } from 'react';
import { r } from '@/utils';

export default function MonitorPage() {

  useEffect(() => {
    redirect(r('/monitor/online'));
  }, []);

  return (
    <>monitor</>
  );
}
