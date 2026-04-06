'use client';

import type { ReactElement } from "react";
import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Divider } from 'antd';
import type { OnlineModel, DescribeForceLogoutRequestParams } from "@repo/apis";
import { onlineApis } from '#src/api';
import { useGlobalState } from '#src/context';
import { useConfirmDelete } from '#src/hooks/modals';
import {
  Actions,
  LinkButton,
} from '#src/components';
import { useList } from './useList';
import { TableActionContainer } from '#src/styles/app.styled';

export interface TableActionsProps {
  record: OnlineModel;
}

export default function TableActions({
  record,
}: TableActionsProps): ReactElement {
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
    await onlineApis.DescribeForceLogout(params);
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
