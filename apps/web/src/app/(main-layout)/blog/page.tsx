import { Metadata } from 'next';
import { DescribeNodeList } from '@repo/apis/server';
import { siteConfig } from '@/constant/config';
import ArticleList from './ArticleList';

export const metadata: Metadata = {
  title: 'IT技术|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function BlogPage() {
  const catVid = 'all';

  const page = 1;
  const pageSize = 10;
  const nodeRes = await DescribeNodeList({
    page,
    pageSize,
  });
  
  return (
    <ArticleList
      catVid={catVid}
      nodeRes={nodeRes}
    />
  );
}
