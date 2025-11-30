import { Metadata } from 'next';

import { siteConfig } from '@/constant/config';
import { MainLayout } from '@/components/layouts';
import BookHome from './BookHome';
import { DescribePoetryListPageData } from '@repo/apis/server'

export const metadata: Metadata = {
  title: '书籍阅读|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function ToolsPage() {
  const filterTags = [
    '花间集',
    '南唐',
    '全唐诗',
    '全宋诗',
    '宋词',
    '楚辞',
    '水墨唐诗',
    '论语',
    '诗经',
    '四书五经',
    '蒙学',
  ];
  const poetryRes = await DescribePoetryListPageData({
    tags: filterTags.join(','),
    poetryAmount: 6,
    chapterAmount: 5,
  });

  return (
    <MainLayout
      activeMenuId='book'
    >
      <BookHome
        books={poetryRes?.entry?.poetry_list}
        chapters={poetryRes?.entry?.chapter_list}
      />
    </MainLayout>
  );
}
