'use client';

import type { TableProps} from 'antd';
import { OnlineModel } from '@/services';

export default function useColumns() {
  const columns: TableProps<OnlineModel>['columns'] = [
    {
      key: 'uid',
      title: '序号',
      dataIndex: 'uid',
    }, 
    {
      key: 'tokenId',
      title: '缓存名称',
      dataIndex: 'tokenId',
    },
    {
      key: 'username',
      title: '备注',
      dataIndex: 'username',
    },
  ];

  return columns;
}
