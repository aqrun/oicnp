import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

const REDIRECT_TO_TOOLS = false;
 
// This function can be marked `async` if using `await` inside
export function middleware(request: NextRequest) {
  if (REDIRECT_TO_TOOLS && !request?.nextUrl?.pathname?.startsWith('/tools/fares')) {
    return NextResponse.redirect(new URL('/tools/fares', request.url));
  }
}
 
// See "Matching Paths" below to learn more
export const config = {
  matcher: [
    '/((?!api|_next/static|_next/image|favicon.ico).*)',
  ],
}

