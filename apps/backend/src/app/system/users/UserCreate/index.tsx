'use client';

import {
  PageTitle,
} from '@/components';
import { setHashState } from '@/utils';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';

export default function UserCreate() {
  const handleBack = useMemoizedFn(() => {
    setHashState({});
  });

  return (
    <Container>
      <PageTitle
        title='创建用户'
        onBack={handleBack}
      />
    </Container>
  );
}
