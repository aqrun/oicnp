'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Button, Divider, Popconfirm } from 'antd';
import {
  RoleModel,
  DescribeDeleteRole,
  DescribeDeleteRoleRequestParams,
} from '@/services';
import { useListStore } from './useListStore';
import { useRouter } from 'next/navigation';
import { r } from '@/utils';
import { useViewStore } from '../detail/useViewStore';
import { useGlobalState } from '@/context';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: RoleModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const { message } = useGlobalState();
  const router = useRouter();
  const setState = useListStore((state) => state.setState);
  const setViewState = useViewStore(state => state.setState);

  const [deleteLoading, setDeleteLoading] = useState(false);

  const handleDelete = useMemoizedFn(async () => {
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

  const handleView = useMemoizedFn(() => {
    // router.push(r(`/system/roles/detail?id=${record?.roleId}`));
    setViewState({
      visible: true,
      roleId: record?.roleId,
    });
  });

  const handleEdit = useMemoizedFn(() => {
    router.push(r(`/system/roles/edit?id=${record?.roleId}`));
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

      <Popconfirm
        placement="topRight"
        title="确定删除？"
        okText="删除"
        cancelText="取消"
        onConfirm={handleDelete}
        okButtonProps={{
          loading: deleteLoading,
        }}
      >
        <Button
          type="text"
          size="small"
          color="danger"
          variant="link"
        >
          删除
        </Button>
      </Popconfirm>
    </TableActionContainer>
  );
}
