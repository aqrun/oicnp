'use server';

import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';

export async function getAuthToken() {
  const cookieStore = await cookies();
  const token = cookieStore.get(SESSION_ID)?.value;
  return token;
}

/**
 * 登陆设置会话
 */
export async function setSession(token: string, expires: Date) {
  const cookieStore = await cookies();

  cookieStore.set(SESSION_ID, token, {
    httpOnly: false,
    secure: false,
    expires,
    sameSite: 'lax',
    path: '/',
  });
}