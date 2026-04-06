'use client';

import type { ReactElement } from "react";

import { useMemoizedFn } from 'ahooks';
import { Button } from 'antd';
import { CLASS_PREFIX } from '#src/constants';

export interface CreateButtonProps {
  label?: string;
  onCreate?: () => void;
}

export function CreateButton({
  label,
  onCreate,
}: CreateButtonProps): ReactElement {
  const handleClick = useMemoizedFn(() => {
    if (typeof onCreate === 'function') {
      onCreate();
    }
  });

  return (
    <Button
      className={`${CLASS_PREFIX}-create-button`}
      onClick={handleClick}
      type="primary"
    >
      {label || '创建'}
    </Button>
  );
}