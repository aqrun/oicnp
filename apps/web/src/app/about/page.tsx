import fs from 'fs';
import { Metadata } from 'next';
import React from 'react';

import { NodeDetailPage } from '@/components/layouts';

import { siteConfig } from '@/constant';
import { pages_path, parseMdx } from '@/utils';

export const metadata: Metadata = {
  title: '关于我',
  description: `关于我 ${siteConfig.description}`,
};

export default async function AboutPage() {
  const file_data = fs.readFileSync(`${pages_path}/about.md`, 'utf-8');

  const Content = await parseMdx(file_data);

  return (
    <NodeDetailPage title='关于我'>
      <Content />
    </NodeDetailPage>
  );
}
