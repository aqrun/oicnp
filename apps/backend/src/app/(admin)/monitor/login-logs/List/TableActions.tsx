'use client';

import { useMemoizedFn } from 'ahooks';
import { Button, Divider } from 'antd';
import {
  LoginLogModel,
} from '@/services';
import { useViewStore } from '../detail/useViewStore';
import { TableActionContainer } from '@/styles/app.styled';

export interface TableActionsProps {
  record: LoginLogModel;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
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
