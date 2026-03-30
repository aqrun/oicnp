import { Metadata } from 'next';
import React from 'react';

import { ClimbDetailPage } from '@/components/climbing';

export const metadata: Metadata = {
  title: '车费估算-秦岭群峰',
};

export default function ClimbPage() {
  return (
    <ClimbDetailPage tab="oil" />
  );
}


