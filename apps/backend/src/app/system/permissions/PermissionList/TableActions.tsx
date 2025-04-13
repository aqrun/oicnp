'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {
  PermissionModel,
  DescribeDeletePermission,
  DescribeDeletePermissionRequestParams,
} from '@/services';
import { useListStore } from './useListStore';
import { useViewStore } from '../view/useViewStore';
import { useEditStore } from '../edit/useEditStore';
import { useConfirmDelete } from '@/hooks/modals';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: PermissionModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const confirmDelete = useConfirmDelete();
  const setState = useListStore((state) => state.setState);
  const setViewState = useViewStore(state => state.setState);
  const setEditState = useEditStore(state => state.setState);

  const [loading, setLoading] = useState(false);

  const deletePermission = useMemoizedFn(async () => {
    setLoading(true);
    const params: DescribeDeletePermissionRequestParams = {
      permissionId: record?.permissionId,
    };
    // 删除
    await DescribeDeletePermission(params);
    // 更新列表
    setState({
      refreshToken: Date.now().toString(),
    });
    setLoading(false);
  });

  const handleDelete = useMemoizedFn(() => {
    confirmDelete({
      title: '删除权限',
      content: `确定删除权限: ${record?.name}?`,
      onOk: deletePermission,
      loading: loading,
    });
  });

  const handleView = useMemoizedFn(() => {
    // router.push(r(`/system/permissions/detail?uid=${record?.permissionId}`));
    setViewState({
      visible: true,
      permissionId: record?.permissionId,
    });
  });

  const handleEdit = useMemoizedFn(() => {
    // router.push(r(`/system/permissions/edit?uid=${record?.permissionId}`));
    setEditState({
      visible: true,
      permissionId: record?.permissionId,
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
