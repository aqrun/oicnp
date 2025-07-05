'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { NodeModel } from '@/services';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<NodeModel>['columns'] = [
    {
      key: 'fileId',
      title: 'ID',
      dataIndex: 'fileId',
      width: 80,
    }, 
    {
      key: 'filename',
      title: '文件名',
      dataIndex: 'filename',
    },
    {
      key: 'uri',
      title: '文件路径',
      dataIndex: 'uri',
      width: 80,
    },
    {
      key: 'storage',
      title: '存储',
      dataIndex: 'storage',
      width: 200,
      render: (value: string) => {
        return value;
      }
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
      render: (value: string, record: NodeModel) => {
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
