import { NextResponse, NextRequest } from 'next/server';
import { SESSION_ID } from '@/constants';
 
// This function can be marked `async` if using `await` inside
export function middleware(request: NextRequest) {
  const response = NextResponse.next();
  
  if (request.nextUrl.pathname.startsWith('/_next')
    || request.nextUrl.pathname.startsWith('/favicon')
  ) {
    return response;
  }

  const isLoginPage = request.nextUrl.pathname.startsWith('/login');
  
  // 获取 cookie 登录状态
  const token = request.cookies.get(SESSION_ID)?.value;

  if (!token && !isLoginPage) {
    return NextResponse.redirect(new URL('/login', request.url));
  }
  // 权限判断
  const isAuth = true;

  if (!isAuth) {
    return NextResponse.redirect(new URL('/home', request.url));
  }

  return response;
}
