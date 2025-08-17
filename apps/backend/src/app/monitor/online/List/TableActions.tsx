'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Divider } from 'antd';
import {
  OnlineModel,
} from '@/services';
import { useGlobalState } from '@/context';
import { useConfirmDelete } from '@/hooks/modals';
import {
  DescribeForceLogout,
  DescribeForceLogoutRequestParams,
} from '@/services';
import {
  Actions,
  LinkButton,
} from '@/components';
import { useList } from './useList';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: OnlineModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const { message } = useGlobalState();
  const confirmDelete = useConfirmDelete();

  const [deleteLoading, setDeleteLoading] = useState(false);
  const { refresh } = useList();

  const forceLogout = useMemoizedFn(async () => {
    setDeleteLoading(true);
    const params: DescribeForceLogoutRequestParams = {
      uid: record.uid,
      tokenId: record.tokenId,
    };
    await DescribeForceLogout(params);
    refresh();
    message.open({
      type: 'success',
      content: '强退成功',
    });
    setDeleteLoading(false);
  });

  const handleDelete = useMemoizedFn(() => {
    confirmDelete({
      title: '强退用户',
      content: `确定强退用户: ${record?.username}?`,
      onOk: forceLogout,
      loading: deleteLoading,
      okText: '强退',
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
