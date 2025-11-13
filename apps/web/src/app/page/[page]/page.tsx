import { Metadata } from 'next';
import { redirect } from 'next/navigation';
import React from 'react';

import { HOME_PAGE_SIZE } from '@/constant';
import { getNodes } from '@/utils';

import { HomePage as BaseHomePage } from '../../HomePage';

export const generateStaticParams = () => {
  const all_nodes = getNodes({
    orderBy: 'date',
  });
  const total = Math.ceil(all_nodes?.length / HOME_PAGE_SIZE);

  const pages = [];

  for (let i = 0; i < total; i++) {
    pages.push(i + 1);
  }

  return pages?.map((page) => {
    return {
      page: `${page}`,
    };
  });
};

export const metadata: Metadata = {
  title: '子十个人博客',
};

export default function HomePage({ params }: { params: { page: string } }) {
  const page = Number(params?.page || 1);
  const pageSize = HOME_PAGE_SIZE;
  const all_nodes = getNodes({
    orderBy: 'date',
  });
  const nodes = getNodes({
    page,
    pageSize,
    orderBy: 'date',
  });

  if (page === 1) {
    return redirect('/');
  }

  return (
    <BaseHomePage
      nodes={nodes}
      page={page}
      pageSize={pageSize}
      total={all_nodes?.length}
    />
  );
}
