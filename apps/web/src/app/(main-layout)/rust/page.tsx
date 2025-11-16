import { Metadata } from 'next';

import { siteConfig } from '@/constant/config';
import BlogLayout from '../blog/BlogLayout';
import {
  ArticleItem,
  SideBar,
} from '@/components/HomePage';
import { Pager1 } from '@/components/pagination';
import {
  DescribeNodeList,
} from '@repo/apis/server';

export const metadata: Metadata = {
  title: 'Rust语言|灵犀纪-心有灵犀，专注技术分享', // `灵犀纪 | ${siteConfig.title}`,
  description: siteConfig.description,
};

export default async function BlogPage() {
  const catVid = 'rust';
  const page = 1;
  const pageSize = 10;
  const nodeRes = await DescribeNodeList({
    page,
    pageSize,
    categoryVids: catVid,
  });

  return (
    <BlogLayout
      catVid={catVid}
    >
      <div className='oic-layout-content flex flex-col'>
        <div className='relative flex flex-wrap flex-row gap-2'>
          {nodeRes?.nodes?.map((item) => {
            return <ArticleItem key={item?.nid} node={item} />;
          })}
        </div>
        <Pager1
          page={page || 0}
          pageSize={pageSize || 10}
          total={nodeRes?.total || 0}
          baseUrl={'/blog'}
        />
      </div>
      <div className='lg:w-80'>
        <SideBar
          hasWeather
          hasTags
        />
      </div>
    </BlogLayout>
  );
}
