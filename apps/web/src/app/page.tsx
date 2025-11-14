import { Metadata } from 'next';

import { HOME_PAGE_SIZE } from '@/constant';
import { siteConfig } from '@/constant/config';
import { getNodes } from '@/utils';
import {
  DescribeNodeList,
} from '@repo/apis/server';

import { HomePage as BaseHomePage } from './HomePage';

export const metadata: Metadata = {
  title: '灵犀纪 - 心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function HomePage() {
  const a = await DescribeNodeList({
    page: 1,
    pageSize: HOME_PAGE_SIZE,
  });
  console.log(a);
  const all_nodes = getNodes({
    orderBy: 'date',
  });
  const nodes = getNodes({
    page: 1,
    pageSize: HOME_PAGE_SIZE,
    orderBy: 'date',
  });

  return (
    <BaseHomePage
      nodes={nodes}
      page={1}
      pageSize={HOME_PAGE_SIZE}
      total={all_nodes?.length}
    />
  );
}
