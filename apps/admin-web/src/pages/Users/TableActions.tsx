import { Button, Divider } from 'antd';
import { UserListData } from '~/api/types';
import { TableActionContainer } from '~/styles/app.styled';

export interface TableActionsProps {
  record: UserListData;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {
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
      >
        删除
      </Button>
    </TableActionContainer>
  );
}
