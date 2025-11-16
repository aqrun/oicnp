import { Metadata } from 'next';

import { siteConfig } from '@/constant/config';
import ToolList from './ToolList';
import { MainLayout } from '@/components/layouts';

export const metadata: Metadata = {
  title: '常用工具|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function ToolsPage() {
  return (
    <MainLayout
      activeMenuId='tool'
    >
      <ToolList />
    </MainLayout>
  );
}
