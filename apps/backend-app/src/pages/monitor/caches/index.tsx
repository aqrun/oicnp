'use client';

import type { ReactElement } from "react";
import { BasicPage } from "#src/components/basic-page";
import {
  PageTitle,
} from '#src/components';
import CacheList from './CacheList';
import KeyList from './KeyList';
import CacheContent from './CacheContent';
import { Container } from './index.styled';

export default function CachesPage(): ReactElement {
  return (
    <BasicPage>
      <Container>
        <PageTitle title="缓存列表" />

        <div className="flex gap-4 h-fit">
          <CacheList />
          <KeyList />
          <CacheContent />
        </div>
      </Container>
    </BasicPage>
  )
}