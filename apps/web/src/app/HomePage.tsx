import React from 'react';

import {
  ArticleItem,
  Footer,
  Header,
  HeaderBg,
  SideBar,
} from '@/components/HomePage';
import { BigNews, News1 } from '@/components/News';
import { Pager1 } from '@/components/pagination';

import { Node } from '@/utils';

export interface HomePageProps {
  nodes: Node[];
  page?: number;
  pageSize?: number;
  total?: number;
}

export const HomePage: React.FC<HomePageProps> = ({
  nodes,
  page,
  pageSize,
  total,
}) => {
  return (
    <main>
      <Header />
      <HeaderBg />
      <BigNews />
      <section className='oic-news-section mt-4'>
        <div className='layout'>
          <News1 />
        </div>
      </section>

      <section className='bg-white'>
        <div className='layout py-12 flex flex-col lg:flex-row gap-8'>
          <div className='flex flex-col w-[calc(100% - 22rem)]'>
            <div className='relative flex flex-wrap flex-row gap-2'>
              {nodes?.map((item) => {
                return <ArticleItem key={item?.data?.title} node={item} />;
              })}
            </div>
            <Pager1
              page={page || 1}
              pageSize={pageSize || 0}
              total={total || 0}
              baseUrl='/page'
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
