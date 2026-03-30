import ArticleDetail from '../../../blog/ArticleDetail';
import { DescribeNodeDetail } from '@repo/apis/server';
import { parseMd } from '@/utils/md';
import { MainLayout } from '@/components/layouts';

export interface ArticleDetailPageProps {
  params: Promise<{
    vid: string;
  }>;
}

export default async function ArticleDetailPage({
  params,
}: ArticleDetailPageProps) {
  const { vid } = await params;
  const nodeRes = await DescribeNodeDetail({
    vid,
    fields: 'body',
  });

  const content = await parseMd(nodeRes?.node?.body || '');

  return (
    <MainLayout
      activeMenuId='blog'
    >
      <ArticleDetail
        node={nodeRes?.node}
        content={content}
      />
    </MainLayout>
  );
}