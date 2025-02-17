'use server';

import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';
import { jwtDecode } from 'jwt-decode';
import { DescribeUser } from '@/services/actions';

/**
 * 登出操作服务端请求
 */
export async function getUser() {
  const cookieStore = await cookies()
  const token = cookieStore.get(SESSION_ID)?.value;

  if (!token) return undefined;

  const decoded = jwtDecode<{
    uid: string;
    uuid: string;
  }>(token);

  const res = await DescribeUser({
    uuid: decoded.uuid,
  });

  return res;
}