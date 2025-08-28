'use server';

import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';

export async function setSession(token: string) {
  const cookieStore = await cookies();

  const expires = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000)
  cookieStore.set(SESSION_ID, token, {
    httpOnly: false,
    secure: false,
    expires: expires,
    sameSite: 'lax',
    path: '/',
  });
}
