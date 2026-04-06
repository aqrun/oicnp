'use client';

import type { ReactElement } from "react";

import { useMemoizedFn } from 'ahooks';
import { Button } from 'antd';
import { Icon } from '../Icon';
import { CLASS_PREFIX } from '#src/constants';

export interface RefreshButtonProps {
  onRefresh?: () => void;
}

export function RefreshButton({
  onRefresh,
}: RefreshButtonProps): ReactElement {
  const handleClick = useMemoizedFn(() => {
    if (typeof onRefresh === 'function') {
      onRefresh();
    }
  });

  return (
    <Button
      className={`${CLASS_PREFIX}-refresh-button`}
      onClick={handleClick}
      type="default"
    >
      <Icon
        icon="ReloadOutlined"
      />
    </Button>
  );
}