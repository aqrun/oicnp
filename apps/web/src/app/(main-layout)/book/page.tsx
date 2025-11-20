import { Metadata } from 'next';

import { siteConfig } from '@/constant/config';
import { MainLayout } from '@/components/layouts';
import { ALL_BOOKS } from '@/content/books';
import BookHome from './BookHome';

export const metadata: Metadata = {
  title: '书籍阅读|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function ToolsPage() {
  return (
    <MainLayout
      activeMenuId='book'
    >
      <BookHome
        books={ALL_BOOKS}
      />
    </MainLayout>
  );
}
