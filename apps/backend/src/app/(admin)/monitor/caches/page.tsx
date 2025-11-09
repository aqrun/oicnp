'use client';

import {
  PageTitle,
} from '@/components';
import CacheList from './CacheList';
import KeyList from './KeyList';
import CacheContent from './CacheContent';
import { Container } from './index.styled';

export default function CachesPage(): JSX.Element {
  return (
    <Container>
      <PageTitle title="缓存列表" />

      <div className="flex gap-4 h-full">
        <CacheList />
        <KeyList />
        <CacheContent />
      </div>
    </Container>
  )
}
