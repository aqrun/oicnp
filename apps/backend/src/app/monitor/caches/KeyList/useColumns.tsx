'use client';

import type { TableProps} from 'antd';
import { CacheModel } from '@/services';
import TableActions from './TableActions';

export default function useColumns() {
  const columns: TableProps<CacheModel>['columns'] = [
    {
      key: 'id',
      title: '序号',
      dataIndex: 'id',
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
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
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
