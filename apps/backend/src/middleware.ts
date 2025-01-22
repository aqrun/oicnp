import { NextResponse, NextRequest } from 'next/server';
import { AUTH_KEY } from '@/constants';
 
// This function can be marked `async` if using `await` inside
export function middleware(request: NextRequest) {
  const response = NextResponse.next();
  
  if (request.nextUrl.pathname.startsWith('/_next')
    || request.nextUrl.pathname.startsWith('/favicon')
  ) {
    return response;
  }
  
  // 获取 cookie 登录状态
  const token = request.cookies.get(AUTH_KEY);
  console.log('token--2222-', request.nextUrl.pathname);
  const isAuth = true;

  if (!isAuth) {
    return NextResponse.redirect(new URL('/home', request.url));
  }
  
  response.headers.set('__name', 'alex');

  return response;
}
