import { Metadata } from 'next';

import { siteConfig } from '@/constant/config';
import { MainLayout } from '@/components/layouts';
import {
  DescribePoetryListWithChapters,
  DescribePoetryListWithChaptersRequestParams,
} from '@repo/apis/server'
import PoetryDetail from '../../PoetryDetail';

export const metadata: Metadata = {
  title: '诗词鉴赏|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function PoetryDetailPage({ params }: { params: { uuid: string } }) {
  const { uuid } = await params;
  const requestParams: DescribePoetryListWithChaptersRequestParams = {
    uuid,
    chapterAmount: 100,
    page: 1,
    pageSize: 1,
    order: 'asc',
    orderBy: 'id',
  };
  
  const poetryRes = await DescribePoetryListWithChapters(requestParams);

  return (
    <MainLayout
      activeMenuId='poetry'
    >
      <PoetryDetail
        poetry={poetryRes?.entry?.poetry_list?.[0]}
        chapters={poetryRes?.entry?.chapter_list}
      />
    </MainLayout>
  );
}
