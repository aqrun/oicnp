import fs from 'fs';
import { Metadata } from 'next';
import React from 'react';

import { NodeDetailPage } from '@/components/layouts';

import { pages_path, parseMdx } from '@/utils';

export const metadata: Metadata = {
  title: '联系我',
};

export default async function AboutPage() {
  const file_data = fs.readFileSync(`${pages_path}/contact.md`, 'utf-8');

  const Content = await parseMdx(file_data);

  return (
    <NodeDetailPage title='联系我'>
      <Content />
    </NodeDetailPage>
  );
}
