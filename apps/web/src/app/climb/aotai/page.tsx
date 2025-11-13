import { Metadata } from 'next';
import React from 'react';

import { ClimbDetailPage } from '@/components/climbing';

export const metadata: Metadata = {
  title: '鳌太路线-秦岭群峰',
};

export default function ClimbPage() {
  return (
    <ClimbDetailPage tab="aotai" />
  );
}

