import { Metadata } from 'next';

import { HOME_PAGE_SIZE } from '@/constant';
import { siteConfig } from '@/constant/config';
import { getNodes } from '@/utils';

import { HomePage as BaseHomePage } from './HomePage';

export const metadata: Metadata = {
  title: '灵犀纪 - 心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default function HomePage() {
  const pageSize = HOME_PAGE_SIZE;
  const all_nodes = getNodes({
    orderBy: 'date',
  });
  const nodes = getNodes({
    page: 1,
    pageSize,
    orderBy: 'date',
  });

  return (
    <BaseHomePage
      nodes={nodes}
      page={1}
      pageSize={pageSize}
      total={all_nodes?.length}
    />
  );
}
