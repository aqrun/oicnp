import { Metadata } from 'next';
import React from 'react';

import { ClimbDetailPage } from '@/components/climbing';

export const metadata: Metadata = {
  title: '秦岭群峰',
};

export default function ClimbPage() {
  return (
    <ClimbDetailPage />
  );
}
