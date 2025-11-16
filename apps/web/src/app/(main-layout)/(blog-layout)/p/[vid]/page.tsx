import ArticleDetail from '../../../blog/ArticleDetail';
import { DescribeNodeDetail } from '@repo/apis/server';

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
  });
  console.log('nodeRes---', nodeRes)
  return (
    <ArticleDetail
      node={nodeRes?.node}
    />
  );
}