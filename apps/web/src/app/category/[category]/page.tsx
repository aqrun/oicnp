import { Metadata } from 'next';
import React from 'react';

import { ArticleListPage } from '@/components/layouts';

import { CATEGORY_PAGE_SIZE, MAIN_MENUS } from '@/constant';
import { getNodes } from '@/utils/blog';

export const generateStaticParams = () => {
  return MAIN_MENUS?.map((item) => {
    return {
      category: item?.vid,
    };
  });
};

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
