import type { Metadata } from "next";
import { MainLayout, AppProvider } from '@/components/layouts';
import FullLoading from '@/components/layouts/FullLoading';
import { getConsoleConfig } from '@/services/apis/common/action';

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
  // 获取全局配置参数
  const configRes = await getConsoleConfig();

  return (
    <html lang="en">
      <body>
        <script
          dangerouslySetInnerHTML={{
            __html: `window.CONSOLE_CONFIG = ${JSON.stringify(configRes?.config || {})};`,
          }}
        />
        
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
