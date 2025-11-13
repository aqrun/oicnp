import React from 'react';

import {
  ArticleItem,
  Footer,
  Header,
  HeaderBg,
  SideBar,
} from '@/components/HomePage';
import { Pager1 } from '@/components/pagination';

import { Node } from '@/utils/blog';

import { BookItem } from '../reading';

export interface ArticleListPageProps {
  nodes: Node[];
  page?: number;
  pageSize?: number;
  total?: number;
  category_vid?: string;
  isBook?: boolean;
}

export const ArticleListPage: React.FC<ArticleListPageProps> = ({
  nodes,
  page,
  pageSize,
  total,
  category_vid,
  isBook,
}) => {
  return (
    <main>
      <Header />
      <HeaderBg />

      <section className='bg-white oic-book-section'>
        <div className='layout py-12 flex flex-col lg:flex-row gap-8'>
          <div className='oic-layout-content flex flex-col w-[calc(100% - 22rem)]'>
            <div className='relative flex flex-wrap flex-row gap-2'>
              {nodes?.map((item) => {
                if (isBook) {
                  return <BookItem key={item?.data?.title} node={item} />;
                }
                return <ArticleItem key={item?.data?.title} node={item} />;
              })}
            </div>
            <Pager1
              page={page || 0}
              pageSize={pageSize || 0}
              total={total || 0}
              baseUrl={`/category/${category_vid}`}
            />
          </div>
          <div className='lg:w-80'>
            <SideBar />
          </div>
        </div>
      </section>

      <Footer />
    </main>
  );
};

