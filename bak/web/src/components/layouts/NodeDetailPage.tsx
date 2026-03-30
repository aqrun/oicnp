import React from 'react';

import { Footer, Header, HeaderBg } from '@/components/HomePage';
import { TableOfContent } from '@/components';

export interface NodeDetailPageProps {
  title: string;
  categoryName?: string;
  categoryUrl?: string;
  date?: string;
  hasArticleMeta?: boolean;
}

export const NodeDetailPage: React.FC<React.PropsWithChildren<NodeDetailPageProps>> = ({
  title,
  children,
  categoryName,
  categoryUrl,
  date,
  hasArticleMeta,
}) => {
  return (
    <main>
      <Header />
      <HeaderBg />

      <section className='bg-white'>
        <div className='layout py-12 flex flex-row gap-8'>
          <div
            className='flex flex-1 flex-col overflow-auto'
            // style={{
            //   width: 'calc(100% - 22rem)',
            // }}
          >
            <h1 className='text-[1.8rem] lg:text-[2.67rem] mb-[1.3rem] text-slate-800 lg:leading-[2.67rem] lg:text-center'>
              {title}
            </h1>
            {hasArticleMeta && (
              <div className='node-metas mb-10 text-gray-400 lg:text-center'>
                <a href={categoryUrl}>
                  <i className='iconfont icon-benshubook122 mr-1' />
                  {categoryName}
                </a>
                <span className='ml-4'>
                  <i className='iconfont icon-date mr-1' />
                  {date}
                </span>
              </div>
            )}
            <article className='oic-article-detail prose lg:prose-lg max-w-full break-words'>
              {children}
            </article>
          </div>
          <TableOfContent />
          {/* sidebar */}
          {/* <div className='lg:w-80'>
            <SideBar />
          </div> */}
        </div>
      </section>
      <Footer />
    </main>
  );
};
