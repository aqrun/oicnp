'use server';

import { DescribeLoginRequestParams } from '@/services';
import { DescribeAuthLogin } from '@/services/actions';
import { cookies } from 'next/headers';
import { SESSION_ID } from '@/constants';
import { redirect } from 'next/navigation';

/**
 * 登陆操作服务端请求
 */
export async function loginAction(params: DescribeLoginRequestParams) {
  const res = await DescribeAuthLogin(params)
  const code = res?.code || '200';

  // 登陆成功设置 cookie
  if (code === '200' && res.token) {
    const cookieStore = await cookies();
    const session = res.token || '';

    const expires = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000)
    cookieStore.set(SESSION_ID, session, {
      httpOnly: true,
      secure: true,
      expires: expires,
      sameSite: 'lax',
      path: '/',
    });
    redirect('/');
  }

  return res;
}
