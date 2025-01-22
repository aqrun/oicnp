import type { Metadata } from "next";
import { AppleLayout, MainLayout } from '@/components/layouts';
import clsx from 'clsx';
import "@/styles/globals.css";
import { A } from '@/styles/a.styled';

export const metadata: Metadata = {
  title: "OICNP 后台管理",
  description: "千里江陵一日还",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const isApple = true;
  const Layout = isApple ? AppleLayout : MainLayout;

  return (
    <html lang="en">
      <body>
        <Layout>
          {children}
        </Layout>
      </body>
    </html>
  );
}
