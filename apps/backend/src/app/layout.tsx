import type { Metadata } from "next";
import { MainLayout, AppProvider } from '@/components/layouts';
import { DescribeMenuTree } from '@/services/actions';

import "@/styles/globals.css";


export const metadata: Metadata = {
  title: "OICNP 后台管理",
  description: "千里江陵一日还",
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const menuRes = await DescribeMenuTree({ mid: 'backend' });

  return (
    <html lang="en">
      <body>
        <AppProvider>
          <MainLayout
            navMenus={menuRes?.children || []}
          >
            {children}
          </MainLayout>
        </AppProvider>
      </body>
    </html>
  );
}
