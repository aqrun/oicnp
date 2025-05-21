'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Divider } from 'antd';
import {
  CategoryModel,
  DescribeDeleteCategory,
  DescribeDeleteCategoryRequestParams,
} from '@/services';
import { useListStore } from './useListStore';
import { useViewStore } from '../detail/useViewStore';
import { useEditStore } from '../edit/useEditStore';
import { useGlobalState } from '@/context';
import { useConfirmDelete } from '@/hooks/modals';
import {
  Actions,
  LinkButton,
} from '@/components';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: CategoryModel;
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
    const params: DescribeDeleteCategoryRequestParams = {
      catId: record?.catId,
    };
    // 删除
    await DescribeDeleteCategory(params);
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
      title: '删除分类',
      content: `确定删除分类: ${record?.catName}?`,
      onOk: deleteRole,
      loading: deleteLoading,
    });
  });

  const handleView = useMemoizedFn(() => {
    setViewState({
      visible: true,
      catId: record?.catId,
    });
  });

  const handleEdit = useMemoizedFn(() => {
    setEditState({
      visible: true,
      catId: record?.catId,
    });
  });

  return (
    <TableActionContainer
      split={<Divider type="vertical" />}
      size="small"
    >
      <Actions>
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
