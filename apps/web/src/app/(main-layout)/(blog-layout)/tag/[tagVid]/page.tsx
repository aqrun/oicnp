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

export interface BlogTagPageProps {
  params: {
    tagVid: string;
  };
}

export default async function BlogTagPage(props: BlogTagPageProps) {
  const { tagVid } = await props.params;

  if (!tagVid || tagVid === 'all') {
    return redirect('/blog');
  }

  const page = 1;
  const pageSize = 10;
  const nodeRes = await DescribeNodeList({
    page,
    pageSize,
    tagVids: tagVid,
  });

  return (
    <MainLayout
      activeMenuId='blog'
    >  
      <ArticleList
        catVid="all"
        tagVid={tagVid}
        nodeRes={nodeRes}
      />
    </MainLayout>
  );
}
