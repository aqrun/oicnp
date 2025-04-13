'use client';

import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {
  UserModel,
  DescribeDeleteUser,
  DescribeDeleteUserRequestParams,
} from '@/services';
import { useUserStore } from './useUserStore';
import { useMutation } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { r } from '@/utils';
import { useConfirmDelete } from '@/hooks/modals';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: UserModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const confirmDelete = useConfirmDelete();
  const router = useRouter();
  const setState = useUserStore((state) => state.setState);

  const m = useMutation({
    mutationFn: (params: DescribeDeleteUserRequestParams) => {
      return DescribeDeleteUser(params);
    },
  });

  const deleteUser = m.mutateAsync;
  const deleteLoading = m.status === 'pending';

  const handleDelete = useMemoizedFn(() => {
    confirmDelete({
      title: '删除用户',
      content: `确定删除用户: ${record?.username}?`,
      onOk: async () => {
        const params: DescribeDeleteUserRequestParams = {
          uid: record?.uid,
        };
        // 删除用户
        await deleteUser(params);
        // 更新列表
        setState({
          refreshToken: Date.now().toString(),
        });
      },
      loading: deleteLoading,
    });
  });

  const handleView = useMemoizedFn(() => {
    router.push(r(`/system/users/detail?uid=${record?.uid}`));
  });

  const handleEdit = useMemoizedFn(() => {
    router.push(r(`/system/users/edit?uid=${record?.uid}`));
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
