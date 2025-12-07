import React from 'react';

import {
  ArticleItem,
  Footer,
  Header,
  HeaderBg,
  SideBar,
} from '@/components/HomePage';
import { BigNews, News1 } from '@/components/News';
import {
  NodeModel,
} from '@repo/apis/client';

export interface HomePageProps {
  nodes: NodeModel[];
}

export const HomePage: React.FC<HomePageProps> = ({
  nodes,
}) => {
  return (
    <main>
      <Header />
      <HeaderBg />
      <BigNews node={nodes?.[0]} />
      <section className='oic-news-section mt-4'>
        <div className='layout'>
          <News1 nodes={nodes?.slice(1, 5)} />
        </div>
      </section>

      <section className='bg-white'>
        <div className='layout py-12 flex flex-col lg:flex-row gap-8'>
          <div className='flex flex-col w-[calc(100% - 22rem)]'>
            <div className='relative flex flex-wrap flex-row gap-2'>
              {nodes?.slice(5)?.map((item) => {
                return <ArticleItem key={item?.nid} node={item} />;
              })}
            </div>
          </div>
          <div className='lg:w-80 lg:min-w-80'>
            <SideBar
              // hasCategories
              hasTags
            />
          </div>
        </div>
      </section>

      <Footer />
    </main>
  );
};
