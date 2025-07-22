'use server';

import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';
import { jwtDecode } from 'jwt-decode';
import { DescribeUserDetail } from '@/services/apis/user/action';

export async function getToken() {
  const cookieStore = await cookies()
  const token = cookieStore.get(SESSION_ID)?.value;
  return token;
}

/**
 * 获取用户信息
 */
export async function getUser() {
  const cookieStore = await cookies()
  const token = cookieStore.get(SESSION_ID)?.value;

  if (!token) return undefined;

  const decoded = jwtDecode<{
    uid: string;
    uuid: string;
  }>(token);

  const res = await DescribeUserDetail({
    uuid: decoded.uuid,
  });

  return res?.user;
}