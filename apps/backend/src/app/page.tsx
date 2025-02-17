'use client';

import { redirect } from 'next/navigation';

export default function Home() {
  redirect('/dashboard');
  return (
    <div>
      index page
    </div>
  );
}
