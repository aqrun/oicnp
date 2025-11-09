'use server';

import { AuthGuard } from '@/components/AuthGuard';

export default async function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <AuthGuard>
      {children}
    </AuthGuard>
  );
}
