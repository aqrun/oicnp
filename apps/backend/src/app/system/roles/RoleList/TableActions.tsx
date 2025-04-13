'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {
  RoleModel,
  DescribeDeleteRole,
  DescribeDeleteRoleRequestParams,
} from '@/services';
import { useListStore } from './useListStore';
import { useViewStore } from '../detail/useViewStore';
import { useEditStore } from '../edit/useEditStore';
import { useGlobalState } from '@/context';
import { useConfirmDelete } from '@/hooks/modals';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: RoleModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const { message } = useGlobalState();
  const confirmDelete = useConfirmDelete();
  const setState = useListStore((state) => state.setState);
  const setViewState = useViewStore(state => state.setState);
  const setEditState = useEditStore(state => state.setState);

  const [deleteLoading, setDeleteLoading] = useState(false);

  const deleteRole = useMemoizedFn(async () => {
    setDeleteLoading(true);
    const params: DescribeDeleteRoleRequestParams = {
      roleId: record?.roleId,
    };
    // 删除
    await DescribeDeleteRole(params);
    // 更新列表
    setState({
      refreshToken: Date.now().toString(),
    });
    message.open({
      type: 'success',
      content: '删除成功',
    });
    setDeleteLoading(false);
  });

  const handleDelete = useMemoizedFn(() => {
    confirmDelete({
      title: '删除角色',
      content: `确定删除角色: ${record?.name}?`,
      onOk: deleteRole,
      loading: deleteLoading,
    });
  });

  const handleView = useMemoizedFn(() => {
    // router.push(r(`/system/roles/detail?id=${record?.roleId}`));
    setViewState({
      visible: true,
      roleId: record?.roleId,
    });
  });

  const handleEdit = useMemoizedFn(() => {
    setEditState({
      visible: true,
      roleId: record?.roleId,
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
