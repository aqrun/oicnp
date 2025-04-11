'use client';

import { useMemoizedFn } from 'ahooks';
import { Button, Divider, Modal } from 'antd';
import {
  MenuModel,
  DescribeDeleteMenu,
  DescribeDeleteMenuRequestParams,
} from '@/services';
import { useMenuStore } from './useMenuStore';
import { useMutation } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { r } from '@/utils';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: MenuModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const router = useRouter();
  const setState = useMenuStore((state) => state.setState);

  const m = useMutation({
    mutationFn: (params: DescribeDeleteMenuRequestParams) => {
      return DescribeDeleteMenu(params);
    },
  });

  const deleteUser = m.mutateAsync;
  const deleteLoading = m.status === 'pending';

  const handleDelete = useMemoizedFn(() => {
    Modal.confirm({
      title: '删除菜单',
      content: `确定删除菜单: ${record?.name}?`,
      okText: '删除',
      okType: 'danger',
      type: 'warning',
      okButtonProps: {
        loading: deleteLoading,
      },
      onOk: async () => {
        const params: DescribeDeleteMenuRequestParams = {
          id: record?.id,
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

  const handleView = useMemoizedFn(() => {
    router.push(r(`/system/menus/detail?id=${record?.id}`));
  });

  const handleEdit = useMemoizedFn(() => {
    router.push(r(`/system/menus/edit?id=${record?.id}`));
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
