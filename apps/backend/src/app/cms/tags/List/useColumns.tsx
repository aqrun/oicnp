'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { TagModel } from '@/services';

export default function useColumns() {
  const columns: TableProps<TagModel>['columns'] = [
    {
      key: 'tagId',
      title: 'ID',
      dataIndex: 'tagId',
      width: 80,
    },
    {
      key: 'tagName',
      title: '标签名称',
      dataIndex: 'tagName',
    },
    {
      key: 'empty',
      title: null,
      dataIndex: 'uid',
      render: () => {
        return <></>;
      }
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'roleId',
      render: (value: string, record: TagModel) => {
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
