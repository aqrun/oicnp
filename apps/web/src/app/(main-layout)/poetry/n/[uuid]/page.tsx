import { Metadata } from 'next';

import { siteConfig } from '@/constant/config';
import { MainLayout } from '@/components/layouts';
import { DescribePoetryListPageData } from '@repo/apis/server'

export const metadata: Metadata = {
  title: '诗词鉴赏|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function PoetryDetailPage({ params }: { params: { uuid: string } }) {
  const { uuid } = await params;
  const poetryRes = await DescribePoetryListPageData({
    poetryAmount: 1,
    chapterAmount: 100,
    uuid,
  });
  console.log('poetryRes---', poetryRes)
  return (
    <MainLayout
      activeMenuId='poetry'
    >
      {uuid}
    </MainLayout>
  );
}
