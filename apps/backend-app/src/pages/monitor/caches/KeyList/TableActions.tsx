'use client';

import type { ReactElement } from "react";

import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {   CacheModel } from "@repo/apis";
import { useList } from '../CacheList/useList';

import { TableActionContainer } from '#src/styles/app.styled';

export interface TableActionsProps {
  record: CacheModel;
}

export default function TableActions({
  record,
}: TableActionsProps): ReactElement {

  const {
    fetchCacheDetail,
  } = useList();

  const handleView = useMemoizedFn(async () => {
    await fetchCacheDetail(record?.cacheKey || '');
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