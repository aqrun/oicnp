'use server';

import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';

/**
 * 登出操作服务端请求
 */
export async function logoutAction() {
  const cookieStore = await cookies()
  cookieStore.delete(SESSION_ID);

  // redirect('/login');
}

export async function getToken() {
  const cookieStore = await cookies()
  const token = cookieStore.get(SESSION_ID)?.value;
  return token;
}
