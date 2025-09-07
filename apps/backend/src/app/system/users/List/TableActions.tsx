'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {
  UserModel,
  DescribeDeleteUser,
  DescribeDeleteUserRequestParams,
} from '@/services';
import { useListStore } from './useListStore';
import { useViewStore } from '../detail/useViewStore';
import { useEditStore } from '../edit/useEditStore';
import { useGlobalState } from '@/context';
import { useConfirmDelete } from '@/hooks/modals';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: UserModel;
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
    const params: DescribeDeleteUserRequestParams = {
      uid: record?.uid,
    };
    // 删除
    const res = await DescribeDeleteUser(params);

    console.log('res-----', res);

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
      title: '删除用户',
      content: `确定删除用户: ${record?.username}?`,
      onOk: deleteRole,
      loading: deleteLoading,
    });
  });

  const handleView = useMemoizedFn(() => {
    // router.push(r(`/system/roles/detail?id=${record?.roleId}`));
    setViewState({
      visible: true,
      uid: record?.uid,
    });
  });

  const handleEdit = useMemoizedFn(() => {
    setEditState({
      visible: true,
      uid: record?.uid,
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
