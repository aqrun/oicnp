'use server';

import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';
import { DescribeAuthInfo } from '@/services/actions';

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

/**
 * 根据cookie获取用户登陆信息
 */
export async function getUserData() {
  const res = await DescribeAuthInfo({});
  return res?.user;
}
