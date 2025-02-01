'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { UserListData } from '@/services/types';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<UserListData>['columns'] = [
    {
      key: 'uid',
      title: 'ID',
      dataIndex: 'uid',
      width: 100,
    },
    {
      key: 'username',
      title: '用户名',
      dataIndex: 'username',
      width: 200,
    },
    {
      key: 'nickname',
      title: '昵称',
      dataIndex: 'nickname',
      width: 200,
    },
    {
      key: 'email',
      title: '邮箱',
      dataIndex: 'email',
      width: 200,
    },
    {
      key: 'phone',
      title: '电话',
      dataIndex: 'phone',
      width: 200,
    },
    {
      key: 'status',
      title: '状态',
      dataIndex: 'status',
      width: 200,
    },
    {
      key: 'created_at',
      title: '创建时间',
      dataIndex: 'created_at',
      width: 200,
      render: (value: string) => {
        return formatDate(value);
      }
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
