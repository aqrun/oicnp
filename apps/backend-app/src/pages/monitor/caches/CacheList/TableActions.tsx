'use client';

import type { ReactElement } from "react";

import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {   CacheScopeModel } from "@repo/apis";
import { useList } from './useList';
import { TableActionContainer } from '#src/styles/app.styled';

export interface TableActionsProps {
  record: CacheScopeModel;
}

export default function TableActions({
  record,
}: TableActionsProps): ReactElement {
  const {
    fetchCacheListByScope,
  } = useList();

  const handleView = useMemoizedFn(async () => {
    await fetchCacheListByScope(record?.scope);
  });

  return (
    <TableActionContainer
      split={<Divider type="vertical" />}
      size="small"
    >
      <Button
        type="text"
        size="small"
        color="default"
        variant="link"
        onClick={handleView}
      >
        查看
      </Button>
    </TableActionContainer>
  );
}