import { Metadata } from 'next';
import React from 'react';

import { ArticleListPage } from '@/components/layouts';

import { getBookList } from '@/utils';


export const metadata: Metadata = {
  title: '阅读小记',
};

export default function ReadingPage() {
  const books = getBookList();

  return (
    <ArticleListPage
      nodes={books}
      isBook
    />
  );
}

