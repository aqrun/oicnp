import { Metadata } from 'next';
import React from 'react';

import { BookDetailPage as BaseNodeDetailPage } from '@/components/layouts';

import { siteConfig } from '@/constant';
import { formatDate,parseMdx } from '@/utils';
import { getBookPages,getCategory } from '@/utils/blog';

export interface MetaProps {
  params: {
    // 书
    book: string;
    // 书页slug
    bookPage: string;
  };
  searchParams: {
    [key: string]: string;
  };
}

export const generateMetadata = async (props: MetaProps): Promise<Metadata> => {
  const book = props?.params?.book;
  const all_nodes = getBookPages(book);
  const node = all_nodes?.[0];
  const category = getCategory(node?.data?.taxonomies?.categories);

  const meta: Metadata = {
    title: `${node?.data?.title} - ${category?.name}`,
    description: node?.data?.description || siteConfig?.description,
  };
  return meta;
};

export interface BookDetailPageProps {
  params: {
    book: string;
  };
  searchParams: {
    [key: string]: string | string[] | undefined;
  };
}

export default async function BookDetailPage(props: BookDetailPageProps) {
  const book = props?.params?.book;
  const all_nodes = getBookPages(book);
  const node = all_nodes?.[0];
  const category = getCategory(node?.data?.taxonomies?.categories);

  const Content = await parseMdx(node?.content || '');

  return (
    <BaseNodeDetailPage
      hasArticleMeta
      title={node?.data?.title || ''}
      categoryName={category?.name}
      categoryUrl={category?.href}
      date={formatDate(node?.data?.date)}
      allPages={all_nodes}
      pageSlug={node?.data?.slug}
    >
      <Content />
    </BaseNodeDetailPage>
  );
}

