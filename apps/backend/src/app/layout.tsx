import type { Metadata } from "next";
import { MainLayout, AppProvider } from '@/components/layouts';
import FullLoading from '@/components/layouts/FullLoading';

import "@/styles/globals.css";


export const metadata: Metadata = {
  title: "OICNP 管理系统",
  description: "千里江陵一日还",
};

export default async function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        <FullLoading />
        <AppProvider>
          <MainLayout>
            {children}
          </MainLayout>
        </AppProvider>
      </body>
    </html>
  );
}
