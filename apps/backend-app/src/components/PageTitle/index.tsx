'use client';

import type { ReactElement } from "react";

import {
  CLASS_PREFIX,
} from '#src/constants';
import { Icon } from '#src/components';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';

export interface PageTitleProps {
  title?: React.ReactNode;
  onBack?: () => void;
}

export function PageTitle({
  title,
  onBack,
}: PageTitleProps): ReactElement {
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
