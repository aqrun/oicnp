import { Metadata } from 'next';
import { redirect } from 'next/navigation';
import { siteConfig } from '@/constant/config';
import ArticleList from '../../../blog/ArticleList';
import { DescribeNodeList } from '@repo/apis/server';
import { MainLayout } from '@/components/layouts';

export const metadata: Metadata = {
  title: 'IT技术|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export interface BlogCategoryPageProps1 {
  params: Promise<{
    catVid: string;
  }>;
}

export default async function BlogCategoryPage({
  params,
}: BlogCategoryPageProps1) {
  const { catVid } = await params;

  if (!catVid || catVid === 'all') {
    return redirect('/blog');
  }

  const page = 1;
  const pageSize = 10;
  const nodeRes = await DescribeNodeList({
    page,
    pageSize,
    categoryVids: catVid,
  });

  return (
    <MainLayout
      activeMenuId='blog'
    >
      <ArticleList
        catVid={catVid}
        nodeRes={nodeRes}
      />
    </MainLayout>
  );
}
