'use client'

import React, { useState } from 'react';
import './globals.css'
import type { Metadata } from 'next'
import MainLayout from './MainLayout';
import UserLogin from '~/containers/UserLogin';
import { RecoilRoot, useRecoilState } from 'recoil';
import { authState } from '~/atoms/authState';

const metadata: Metadata = {
  title: 'Create Next App111',
  description: 'Generated by create next app',
}

/**
 * main layout widget
 */
const LayoutWidget: React.FC<React.PropsWithChildren<{}>> = ({
  children,
}) => {
  const [auth] = useRecoilState(authState);

  if (!auth.user) {
    return (
      <UserLogin />
    );
  }

  return (
    <MainLayout>
      {children}
    </MainLayout>
  );
};

/**
 * main layout
 */
export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  const user = false;

  return (
    <html lang="en">
      <body>
        <RecoilRoot>
          <LayoutWidget>{children}</LayoutWidget>
        </RecoilRoot>
      </body>
    </html>
  )
}
