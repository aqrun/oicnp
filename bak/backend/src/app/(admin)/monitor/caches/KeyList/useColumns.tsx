'use client';

import type { TableProps} from 'antd';
import { CacheModel } from '@/services';
import TableActions from './TableActions';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<CacheModel>['columns'] = [
    {
      key: 'id',
      title: 'ID',
      dataIndex: 'id',
      width: 60,
    }, 
    {
      key: 'cacheKey',
      title: '缓存键名',
      dataIndex: 'cacheKey',
    },
    {
      key: 'createdAt',
      title: '创建时间',
      dataIndex: 'createdAt',
      render: (value: string) => {
        return formatDate(value);
      }
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 80,
      dataIndex: 'scope',
      render: (value: string, record: CacheModel) => {
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
