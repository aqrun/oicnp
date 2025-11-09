'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { PositionModel } from '@/services';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<PositionModel>['columns'] = [
    {
      key: 'positionId',
      title: 'ID',
      dataIndex: 'positionId',
      width: 80,
    }, 
    {
      key: 'vid',
      title: 'VID',
      dataIndex: 'vid',
      width: 80,
    },
    {
      key: 'name',
      title: '名称',
      dataIndex: 'name',
    },
    {
      key: 'weight',
      title: '权重',
      dataIndex: 'weight',
    },
    {
      key: 'remark',
      title: '备注',
      dataIndex: 'remark',
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
      render: (value: string, record: PositionModel) => {
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
