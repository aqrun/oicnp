import React from 'react';

import clsx from 'clsx';

import { Footer, Header, HeaderBg, SideBar } from '@/components/HomePage';
import { Node } from '@/utils';
import { TableOfContent2 } from '@/components/TableOfContent/TableOfContent2';

export interface BookDetailPageProps {
  title: string;
  categoryName?: string;
  categoryUrl?: string;
  date?: string;
  hasArticleMeta?: boolean;
  allPages?: Node[];
  pageSlug?: string;
}

export const BookDetailPage: React.FC<React.PropsWithChildren<BookDetailPageProps>> = ({
  title,
  children,
  categoryName,
  categoryUrl,
  date,
  hasArticleMeta,
  allPages,
  pageSlug,
}) => {
  return (
    <main>
      <Header />
      <HeaderBg />

      <section className='bg-white'>
        <div className='max-w-[90rem] mx-auto px-4 lg:px-8 py-12 flex flex-col lg:flex-row gap-8'>
          <div className="side-a w-[19rem] hidden1 lg:block">
            <h4>
              目录
            </h4>
            <div>
              <ul>
                {allPages?.map((item) => {
                  const cls = pageSlug === item?.data?.slug ? 'active text-indigo-500' : '';
                  return (
                    <li key={item?.data?.slug}>
                      <a
                        className={clsx(cls)}
                        href={`/reading/${item?.data?.book}/${item?.data?.slug}`}
                      >
                        {item?.data?.title}
                      </a>
                    </li>
                  );
                })}
              </ul>
            </div>
          </div>
          <div
            className='flex flex-1 flex-col overflow-auto'
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
          
          {/* sidebar */}
          <div className='hidden lg:block lg:w-80'>
            <TableOfContent2 />
            <SideBar />
          </div>
        </div>
      </section>
      <Footer />
    </main>
  );
};

