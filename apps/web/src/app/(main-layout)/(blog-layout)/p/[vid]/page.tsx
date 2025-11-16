import ArticleDetail from '../../../blog/ArticleDetail';
import { DescribeNodeDetail } from '@repo/apis/server';
import { parseMd } from '@/utils/md';

export interface ArticleDetailPageProps {
  params: {
    vid: string;
  };
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
    <ArticleDetail
      node={nodeRes?.node}
      content={content}
    />
  );
}