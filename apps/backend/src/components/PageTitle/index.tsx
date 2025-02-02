'use client';

import {
  CLASS_PREFIX,
} from '@/constants';
import { Icon } from '@/components';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';

export interface PageTitleProps {
  title?: React.ReactNode;
  onBack?: () => void;
}

export function PageTitle({
  title,
  onBack,
}: PageTitleProps): JSX.Element {
  const handleBack = useMemoizedFn(() => {
    if (typeof onBack !== 'undefined') {
      onBack();
    }
  });

  return (
    <Container
      className={`${CLASS_PREFIX}-pageTitle-container`}
    >
      {Boolean(onBack) && (
        <div
          className={`${CLASS_PREFIX}-icon-back`}
          onClick={handleBack}
        >
          <Icon icon="ArrowLeftOutlined" />
        </div>
      )}
      <h2>
        {title}
      </h2>
    </Container>
  );
}
