'use server';

import { redirect } from 'next/navigation';
import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';

/**
 * 登出操作服务端请求
 */
export async function logoutAction() {
  const cookieStore = await cookies()
  cookieStore.delete(SESSION_ID);

  redirect('/login');
}