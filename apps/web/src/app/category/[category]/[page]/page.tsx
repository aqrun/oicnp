import { Metadata } from 'next';
import React from 'react';

import { ArticleListPage } from '@/components/layouts';

import { CATEGORY_PAGE_SIZE, MAIN_MENUS } from '@/constant';
import { getNodes } from '@/utils/blog';

export interface MetaProps {
  params: {
    category: string;
  };
  searchParams: {
    [key: string]: string;
  };
}

export const generateMetadata = async (props: MetaProps): Promise<Metadata> => {
  const params = await props.params;
  const cat_vid = params?.category;
  const category = MAIN_MENUS?.find((item) => item?.vid === cat_vid);

  return {
    title: category?.name,
  };
};

export const generateStaticParams = () => {
  const allParams: {
    category: string;
    page: string;
  }[] = [];

  MAIN_MENUS.forEach((menu) => {
    const all_nodes = getNodes({
      category: menu?.vid,
      orderBy: 'date',
    });
    const total = Math.ceil(all_nodes?.length / CATEGORY_PAGE_SIZE);

    for (let i = 0; i < total; i++) {
      // 每个分类对应的所有页
      allParams.push({
        category: menu?.vid,
        page: `${i + 1}`,
      });
    }
  });

  return allParams;
};

export interface CategoryPageProps {
  params: {
    category: string;
    page: string;
  };
  searchParams: {
    [key: string]: string | string[] | undefined;
  };
}

export default function CategoryPage(props: CategoryPageProps) {
  const category_vid = props?.params?.category;
  const page = Number(props?.params?.page || 1);
  const pageSize = CATEGORY_PAGE_SIZE;
  const all_nodes = getNodes({
    category: category_vid,
    orderBy: 'date',
  });
  const nodes = getNodes({
    category: category_vid,
    page,
    pageSize: 5,
    orderBy: 'date',
  });

  return (
    <ArticleListPage
      nodes={nodes}
      page={page}
      pageSize={pageSize}
      total={all_nodes?.length}
      category_vid={category_vid}
    />
  );
}
