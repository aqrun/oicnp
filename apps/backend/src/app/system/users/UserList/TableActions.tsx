'use client';

import { useMemoizedFn } from 'ahooks';
import { Button, Divider, Modal } from 'antd';
import {
  UserListData,
  DescribeDeleteUser,
  DescribeDeleteUserRequestParams,
} from '@/services';
import { useUserStore } from './useUserStore';
import { useMutation } from '@tanstack/react-query';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: UserListData;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const setState = useUserStore((state) => state.setState);

  const m = useMutation({
    mutationFn: (params: DescribeDeleteUserRequestParams) => {
      return DescribeDeleteUser(params);
    },
  });

  const deleteUser = m.mutateAsync;
  const deleteLoading = m.status === 'pending';

  const handleDelete = useMemoizedFn(() => {
    Modal.confirm({
      title: '删除用户',
      content: `确定删除用户: ${record?.username}?`,
      okText: '删除',
      okType: 'danger',
      type: 'warning',
      okButtonProps: {
        loading: deleteLoading,
      },
      onOk: async () => {
        const params: DescribeDeleteUserRequestParams = {
          uid: record?.id,
        };
        // 删除用户
        await deleteUser(params);
        // 更新列表
        setState({
          refreshToken: Date.now().toString(),
        });
      }
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
      >
        查看
      </Button>
      <Button
        type="text"
        size="small"
        color="primary"
        variant="link"
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
