'use client'

import React, { useEffect, useState } from 'react';
import './globals.css'
import type { Metadata } from 'next'
import { RecoilRoot } from 'recoil';
import { useRouter, usePathname } from 'next/navigation';
import {
  SyncOutlined,
} from '@ant-design/icons';
import {
  QueryClientProvider,
} from '@tanstack/react-query'
import { useAuthState } from '~/hooks';
import { r, queryClient } from '~/utils';
import MainLayout from './main-layout';

// eslint-disable-next-line no-unused-vars, @typescript-eslint/no-unused-vars -- metadata
const metadata: Metadata = {
  title: 'Create Next App111',
  description: 'Generated by create next app',
}

/**
 * main layout widget
 */
function LayoutWidget ({
  children,
}: React.PropsWithChildren): JSX.Element {
  const [auth] = useAuthState(true);
  const router = useRouter();
  const pathname = usePathname();
  const [initLoading, setInitLoading] = useState(true);

  useEffect(() => {
    setInitLoading(false);
  }, []);

  useEffect(() => {
    if (!auth.user) {
      router.push(r('/login'));
    } else if (pathname === '/login') {
      router.push(r('/welcome'));
    }
  }, [auth, router, pathname]);

  if (initLoading) {
    return (
      <div className="oicnp-loading-container">
        <SyncOutlined spin />
      </div>
    );
  }

  if (!auth.user) {
    return (
      <>
        {children}
      </>
    );
  }

  return (
    <MainLayout>
      {children}
    </MainLayout>
  );
}

/**
 * main layout
 */
export default function RootLayout ({
  children,
}: React.PropsWithChildren): JSX.Element {
  return (
    <html lang="en">
      <body>
        <RecoilRoot>
          <QueryClientProvider client={queryClient}>
            <LayoutWidget>
              {children}
            </LayoutWidget>
          </QueryClientProvider>
        </RecoilRoot>
      </body>
    </html>
  );
}
