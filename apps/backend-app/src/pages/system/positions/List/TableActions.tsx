'use client';

import type { ReactElement } from "react";

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import type { PositionModel, DescribeDeletePositionRequestParams } from '@repo/apis';
import { positionApis } from '#src/api';
import { useListStore } from './useListStore';
import { useViewStore } from '../detail/useViewStore';
import { useEditStore } from '../edit/useEditStore';
import { useGlobalState } from '#src/context';
import { useConfirmDelete } from '#src/hooks/modals';
import { TableActionContainer } from '#src/styles/app.styled';

export interface TableActionsProps {
  record: PositionModel;
}

export default function TableActions({
  record,
}: TableActionsProps): ReactElement {
  const { message } = useGlobalState();
  const confirmDelete = useConfirmDelete();
  const setState = useListStore((state) => state.setState);
  const setViewState = useViewStore(state => state.setState);
  const setEditState = useEditStore(state => state.setState);

  const [deleteLoading, setDeleteLoading] = useState(false);

  const deleteRole = useMemoizedFn(async () => {
    setDeleteLoading(true);
    const params: DescribeDeletePositionRequestParams = {
      positionId: record?.positionId,
    };
    // 删除
    const res = await positionApis.DescribeDeletePosition(params);
    const code = res?.code ?? '200';

    if (code === '200') {
      // 更新列表
      setState({
        refreshToken: Date.now().toString(),
      });
      message.open({
        type: 'success',
        content: '删除成功',
      });
    }
    setDeleteLoading(false);
  });

  const handleDelete = useMemoizedFn(() => {
    confirmDelete({
      title: '删除职位',
      content: `确定删除职位: ${record?.name}?`,
      onOk: deleteRole,
      loading: deleteLoading,
    });
  });

  const handleView = useMemoizedFn(() => {
    // router.push(r(`/system/roles/detail?id=${record?.roleId}`));
    setViewState({
      visible: true,
      positionId: record?.positionId,
    });
  });

  const handleEdit = useMemoizedFn(() => {
    setEditState({
      visible: true,
      positionId: record?.positionId,
    });
  });

  return (
    <TableActionContainer
      split={<Divider type="vertical" />}
      size="small"
    >
      <Button
        type="text"
        size="small"
        color="primary"
        variant="link"
        onClick={handleView}
      >
        查看
      </Button>
      <Button
        type="text"
        size="small"
        color="primary"
        variant="link"
        onClick={handleEdit}
      >
        编辑
      </Button>
      <Button
        type="text"
        size="small"
        color="danger"
        variant="link"
        onClick={handleDelete}
      >
        删除
      </Button>
    </TableActionContainer>
  );
}
