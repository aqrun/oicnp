'use client';

import { useMemoizedFn } from 'ahooks';
import { Button } from 'antd';
import { CLASS_PREFIX } from '@/constants';

export interface ExpandButtonProps {
  onExpand?: () => void;
}

export function ExpandButton({
  onExpand,
}: ExpandButtonProps): JSX.Element {
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