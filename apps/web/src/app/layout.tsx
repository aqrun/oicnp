import { Metadata } from 'next';
import * as React from 'react';
import Script from 'next/script';
import { Analytics } from '@vercel/analytics/react';

import '@/styles/globals.css';
// !STARTERCONF This is for demo purposes, remove @/styles/colors.css import immediately
import '@/styles/colors.css';

import { siteConfig } from '@/constant/config';

// !STARTERCONF Change these default meta
// !STARTERCONF Look at @/constant/config to change them
export const metadata: Metadata = {
  metadataBase: new URL(siteConfig.url),
  title: {
    default: siteConfig.title,
    template: `%s | ${siteConfig.title}`,
  },
  description: siteConfig.description,
  robots: { index: true, follow: true },
  // !STARTERCONF this is the default favicon, you can generate your own from https://realfavicongenerator.net/
  // ! copy to /favicon folder
  icons: {
    icon: '/favicon/favicon.ico',
  },
  manifest: `/favicon/site.webmanifest`,
  openGraph: {
    url: siteConfig.url,
    title: siteConfig.title,
    description: siteConfig.description,
    siteName: siteConfig.title,
    images: [`${siteConfig.url}/images/og.jpg`],
    type: 'website',
    locale: 'en_US',
  },
  twitter: {
    card: 'summary_large_image',
    title: siteConfig.title,
    description: siteConfig.description,
    images: [`${siteConfig.url}/images/og.jpg`],
    // creator: '@th_clarence',
  }
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="zh-CN" suppressHydrationWarning>
      <body suppressHydrationWarning data-url={process.env.URL_API}>
        <script
          dangerouslySetInnerHTML={{
            __html: `
              window.a = ${JSON.stringify(process.env)};
            `,
          }}
        />
        <script
          dangerouslySetInnerHTML={{
            __html: `
              (function() {
                try {
                  if (document.readyState === 'loading') {
                    document.addEventListener('DOMContentLoaded', function() {
                      document.documentElement.classList.add('loaded');
                    });
                  } else {
                    document.documentElement.classList.add('loaded');
                  }
                } catch (e) {}
              })();
            `,
          }}
        />
        <Script src="/baidu.js?v1" strategy="beforeInteractive" />
        {children}
        <Analytics />
      </body>
    </html>
  );
}
