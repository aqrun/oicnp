'use client';

import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {
  CacheModel,
} from '@/services';
import { useList } from '../CacheList/useList';

import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: CacheModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {

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
