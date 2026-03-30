import { Metadata } from 'next';

import { siteConfig } from '@/constant/config';
import { MainLayout } from '@/components/layouts';
import BookList from '../../BookList';
import {
  DescribePoetryListWithChapters,
  DescribePoetryListWithChaptersRequestParams,
} from '@repo/apis/server'
import { BOOK_CATEGORIES } from '@/content/books/base';

export const metadata: Metadata = {
  title: '诗词鉴赏|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function PoetryCategoryPage({ params }: { params: Promise<{ catId: string }> }) {
  const { catId } = await params;
  const category = BOOK_CATEGORIES.find((item) => item?.id === catId);
  const requestParams: DescribePoetryListWithChaptersRequestParams = {
    tags: category?.tags?.join(','),
    chapterAmount: 5,
    page: 1,
    pageSize: 10,
    order: 'asc',
    orderBy: 'id',
  };

  if (category?.dynasty) {
    requestParams.dynasty = category?.dynasty;
  }
  
  const poetryRes = await DescribePoetryListWithChapters(requestParams);
  const needLoadMore = poetryRes?.entry?.poetry_list?.length < poetryRes?.entry?.total;

  return (
    <MainLayout
      activeMenuId='poetry'
    >
      <BookList
        catVid={catId}
        books={poetryRes?.entry?.poetry_list}
        chapters={poetryRes?.entry?.chapter_list}
        needLoadMore={needLoadMore}
      />
    </MainLayout>
  );
}
