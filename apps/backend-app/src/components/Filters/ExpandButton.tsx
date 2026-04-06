'use client';

import type { ReactElement } from "react";

import { useMemoizedFn } from 'ahooks';
import { Button } from 'antd';
import { CLASS_PREFIX } from '#src/constants';

export interface ExpandButtonProps {
  onExpand?: () => void;
}

export function ExpandButton({
  onExpand,
}: ExpandButtonProps): ReactElement {
  const handleClick = useMemoizedFn(() => {
    if (typeof onExpand === 'function') {
      onExpand();
    }
  });

  return (
    <Button
      className={`${CLASS_PREFIX}-expand-button`}
      onClick={handleClick}
    >
      展开/收起
    </Button>
  );
}