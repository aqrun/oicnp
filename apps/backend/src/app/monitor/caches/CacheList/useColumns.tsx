'use client';

import type { TableProps} from 'antd';
import { CacheScopeModel } from '@/services';
import TableActions from './TableActions';

export default function useColumns() {
  const columns: TableProps<CacheScopeModel>['columns'] = [
    {
      key: 'scope',
      title: '缓存Key',
      dataIndex: 'scope',
    },
    {
      key: 'label',
      title: '缓存分类',
      dataIndex: 'label',
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'scope',
      render: (value: string, record: CacheScopeModel) => {
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
