import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { UserListData } from '~/api/types';

export default function useColumns() {
  const columns: TableProps<UserListData>['columns'] = [
    {
      key: 'id',
      title: 'ID',
      dataIndex: 'id',
      width: 100,
    },
    {
      key: 'username',
      title: '用户名',
      dataIndex: 'username',
      width: 200,
    },
    {
      key: 'phone',
      title: '电话',
      dataIndex: 'phone',
      width: 200,
    },
    {
      key: 'id',
      title: null,
      dataIndex: 'id',
      render: () => null,
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'id',
      render: (value: string, record: UserListData) => {
        return (
          <TableActions
            record={record}
          />
        );
      }
    }
  ];

  return columns;
}
