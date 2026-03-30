'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Divider } from 'antd';
import { Actions, LinkButton } from '@/components';
import {
  PermissionModel,
  DescribeDeletePermission,
  DescribeDeletePermissionRequestParams,
} from '@/services';
import { useListStore } from './useListStore';
import { useViewStore } from '../view/useViewStore';
import { useEditStore } from '../edit/useEditStore';
import { useConfirmDelete } from '@/hooks/modals';
import { useCreateStore } from '../create/useCreateStore';
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
  const setCreateState = useCreateStore(state => state.setState);
  const [loading, setLoading] = useState(false);

  const deletePermission = useMemoizedFn(async () => {
    setLoading(true);
    const params: DescribeDeletePermissionRequestParams = {
      permissionId: record?.permissionId,
    };
    // 删除
    const res = await DescribeDeletePermission(params);
    const code = res?.code ?? '200';

    if (code === '200') {
      // 更新列表
      setState({
        refreshToken: Date.now().toString(),
      });
    }
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

  const handleInsert = useMemoizedFn(() => {
    setCreateState({
      visible: true,
      initPid: record?.permissionId,
      contentType: 'create',
    });
  });

  return (
    <TableActionContainer
      split={<Divider type="vertical" />}
      size="small"
    >
      <Actions>
        <LinkButton
          key="insert"
          onClick={handleInsert}
        >
          新增
        </LinkButton>
        <LinkButton
          key="view"
          onClick={handleView}
        >
          查看
        </LinkButton>
        <LinkButton
          key="edit"
          onClick={handleEdit}
        >
          编辑
        </LinkButton>
        <LinkButton
          key="delete"
          danger
          onClick={handleDelete}
        >
          删除
        </LinkButton>
      </Actions>
    </TableActionContainer>
  );
}
