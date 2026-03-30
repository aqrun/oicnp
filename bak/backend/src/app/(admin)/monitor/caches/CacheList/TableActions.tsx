'use client';

import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {
  CacheScopeModel,
} from '@/services';
import { useList } from './useList';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: CacheScopeModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
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
