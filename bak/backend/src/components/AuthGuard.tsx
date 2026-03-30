'use server';

import { getAuthToken } from '@/services/auth';
import { redirect } from 'next/navigation';

export interface AuthGuardProps {
  children: React.ReactNode;
  next?: string;
}

export async function AuthGuard({
  children,
  next,
}: AuthGuardProps): Promise<JSX.Element> {
  const token = await getAuthToken();

  if (!token) {
    const url = next ? `/login?next=${next}` : '/login';
    return redirect(url) as JSX.Element;
  }

  return children as JSX.Element;
}