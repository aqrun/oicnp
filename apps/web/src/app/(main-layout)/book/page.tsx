import { Metadata } from 'next';

import { siteConfig } from '@/constant/config';

export const metadata: Metadata = {
  title: '书籍阅读|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function ToolsPage() {
  return (
    <div>
      <h1>书籍阅读</h1>
    </div>
  );
}
