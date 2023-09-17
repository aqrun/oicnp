import { NextResponse, NextRequest } from 'next/server';
import fs from 'fs';

export const GET = async (request: NextRequest) => {
  // console.log('re---', request);
  console.log('re---', request.nextUrl.searchParams.get('file'));
  const mainLayout = fs.readFileSync('./src/app/MainLayout.tsx', { encoding: 'utf-8' });

  const data = {
    name: 'alex',
    mainLayout,
  };

  return NextResponse.json(data);
};
