'use client';

import type { ReactElement } from "react";

import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {   LoginLogModel } from "@repo/apis";
import { useViewStore } from '../detail/useViewStore';
import { TableActionContainer } from '#src/styles/app.styled';

export interface TableActionsProps {
  record: LoginLogModel;
}

export default function TableActions({
  record,
}: TableActionsProps): ReactElement {
  const setViewState = useViewStore(state => state.setState);

  const handleView = useMemoizedFn(() => {
    // router.push(r(`/system/roles/detail?id=${record?.roleId}`));
    setViewState({
      visible: true,
      loginLogId: record?.id,
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
    </TableActionContainer>
  );
}