'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Divider } from 'antd';
import {
  CronModel,
} from '@/services';
import { useGlobalState } from '@/context';
import { useConfirmDelete } from '@/hooks/modals';
import {
  Actions,
  LinkButton,
} from '@/components';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: CronModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const { message } = useGlobalState();
  const confirmDelete = useConfirmDelete();

  const [deleteLoading, setDeleteLoading] = useState(false);

  const forceLogout = useMemoizedFn(async () => {
    setDeleteLoading(true);
    message.open({
      type: 'success',
      content: '删除成功',
    });
    setDeleteLoading(false);
  });

  const handleDelete = useMemoizedFn(() => {
    confirmDelete({
      title: '删除任务',
      content: `确定删除任务: ${record?.name}?`,
      onOk: forceLogout,
      loading: deleteLoading,
    });
  });

  return (
    <TableActionContainer
      split={<Divider type="vertical" />}
      size="small"
    >
      <Actions>
        <LinkButton
          key="delete"
          onClick={handleDelete}
        >
          强退
        </LinkButton>
      </Actions>
    </TableActionContainer>
  );
}
