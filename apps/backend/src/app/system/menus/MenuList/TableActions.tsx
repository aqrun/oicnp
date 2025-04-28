'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Divider } from 'antd';
import { Actions, LinkButton } from '@/components';
import {
  MenuModel,
  DescribeDeleteMenu,
  DescribeDeleteMenuRequestParams,
} from '@/services';
import { useListStore } from './useListStore';
import { useViewStore } from '../view/useViewStore';
import { useEditStore } from '../edit/useEditStore';
import { useConfirmDelete } from '@/hooks/modals';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: MenuModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
  const confirmDelete = useConfirmDelete();
  const setState = useListStore((state) => state.setState);
  const setViewState = useViewStore(state => state.setState);
  const setEditState = useEditStore(state => state.setState);

  const [delLoading, setDelLoading] = useState(false);

  const deletePermission = useMemoizedFn(async () => {
    setDelLoading(true);
    const params: DescribeDeleteMenuRequestParams = {
      id: record?.id,
    };
    // 删除
    await DescribeDeleteMenu(params);
    // 更新列表
    setState({
      refreshToken: Date.now().toString(),
    });
    setDelLoading(false);
  });

  const handleDelete = useMemoizedFn(() => {
    confirmDelete({
      title: '删除菜单',
      content: `确定删除菜单: ${record?.name}?`,
      onOk: deletePermission,
      loading: delLoading,
    });
  });

  const handleView = useMemoizedFn(() => {
    setViewState({
      visible: true,
      menuId: record?.id,
    });
  });

  const handleEdit = useMemoizedFn(() => {
    setEditState({
      visible: true,
      menuId: record?.id,
    });
  });

  return (
    <TableActionContainer
      split={<Divider type="vertical" />}
      size="small"
    >
      <Actions threshold={3} >
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
