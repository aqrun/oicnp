'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { DepartmentModel } from '@/services';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<DepartmentModel>['columns'] = [
    {
      key: 'id',
      title: 'ID',
      dataIndex: 'id',
      width: 80,
    }, 
    // {
    //   key: 'pid',
    //   title: '父级',
    //   dataIndex: 'pid',
    //   width: 80,
    // },
    {
      key: 'name',
      title: '名称',
      dataIndex: 'name',
    },
    {
      key: 'vid',
      title: 'VID',
      dataIndex: 'vid',
      width: 80,
    },
    {
      key: 'weight',
      title: '权重',
      dataIndex: 'weight',
    },
    {
      key: 'leader',
      title: '负责人',
      dataIndex: 'leader',
    },
    {
      key: 'phone',
      title: '电话',
      dataIndex: 'phone',
    },
    {
      key: 'email',
      title: '邮箱',
      dataIndex: 'email',
    },
    {
      key: 'status',
      title: '状态',
      dataIndex: 'status',
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
      dataIndex: 'nid',
      render: () => {
        return <></>;
      }
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'nid',
      render: (value: string, record: DepartmentModel) => {
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
