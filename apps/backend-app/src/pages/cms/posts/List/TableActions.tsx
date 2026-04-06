'use client';

import type { ReactElement } from "react";

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Divider } from 'antd';
import {   NodeModel, DescribeDeleteNodeRequestParams } from "@repo/apis";
import { nodeApis } from "#src/api";
import { useListStore } from './useListStore';
import { useViewStore } from '../detail/useViewStore';
import { useEditStore } from '../edit/useEditStore';
import { useGlobalState } from '#src/context';
import { useConfirmDelete } from '#src/hooks/modals';
import {
  Actions,
  LinkButton,
} from '#src/components';
import { TableActionContainer } from '#src/styles/app.styled';

export interface TableActionsProps {
  record: NodeModel;
}

export default function TableActions({
  record,
}: TableActionsProps): ReactElement {
  const { message } = useGlobalState();
  const confirmDelete = useConfirmDelete();
  const setState = useListStore((state) => state.setState);
  const setViewState = useViewStore(state => state.setState);
  const setEditState = useEditStore(state => state.setState);

  const [deleteLoading, setDeleteLoading] = useState(false);

  const deleteNode = useMemoizedFn(async () => {
    setDeleteLoading(true);
    const params: DescribeDeleteNodeRequestParams = {
      nid: record?.nid,
    };
    // 删除
    const res = await nodeApis.DescribeDeleteNode(params);
    const code = res?.code ?? '200';

    if (code === '200') {
      // 更新列表
      setState({
        refreshToken: Date.now().toString(),
      });
      message.open({
        type: 'success',
        content: '删除成功',
      });
    }
    
    setDeleteLoading(false);
  });

  const handleDelete = useMemoizedFn(() => {
    confirmDelete({
      title: '删除内容',
      content: `确定删除内容: ${record?.title}?`,
      onOk: deleteNode,
      loading: deleteLoading,
    });
  });

  const handleView = useMemoizedFn(() => {
    setViewState({
      visible: true,
      nid: record?.nid,
    });
  });

  const handleEdit = useMemoizedFn(() => {
    setEditState({
      visible: true,
      nid: record?.nid,
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