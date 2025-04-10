'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { RoleModel } from '@/services';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<RoleModel>['columns'] = [
    {
      key: 'roleId',
      title: 'roleId',
      dataIndex: 'roleId',
      width: 80,
    },
    {
      key: 'vid',
      title: 'vid',
      dataIndex: 'vid',
      width: 120,
    },
    {
      key: 'name',
      title: '名称',
      dataIndex: 'name',
      width: 200,
    },
    {
      key: 'status',
      title: '状态',
      dataIndex: 'status',
      width: 120,
    },
    {
      key: 'weight',
      title: '排序',
      dataIndex: 'weight',
      width: 80,
    },
    {
      key: 'remark',
      title: '描述',
      dataIndex: 'remark',
      width: 200,
    },
    {
      key: 'createdAt',
      title: '创建时间',
      dataIndex: 'createdAt',
      width: 200,
      render: (value: string) => {
        return formatDate(value);
      }
    },
    {
      key: 'empty',
      title: null,
      dataIndex: 'uid',
      render: (value: string) => {
        return <></>;
      }
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'roleId',
      render: (value: string, record: RoleModel) => {
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
