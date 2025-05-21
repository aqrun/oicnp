'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { CategoryModel } from '@/services';

export default function useColumns() {
  const columns: TableProps<CategoryModel>['columns'] = [
    {
      key: 'catId',
      title: 'ID',
      dataIndex: 'catId',
      width: 80,
    },
    {
      key: 'catVid',
      title: 'VID',
      dataIndex: 'catVid',
      width: 120,
    },
    {
      key: 'catName',
      title: '分类名称',
      dataIndex: 'catName',
    },
    {
      key: 'catDesc',
      title: '分类描述',
      dataIndex: 'catDesc',
    },
    {
      key: 'weight',
      title: '权重',
      dataIndex: 'weight',
      width: 80,
    },
    {
      key: 'empty',
      title: null,
      dataIndex: 'catId',
      render: () => {
        return <></>;
      }
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'catId',
      render: (value: string, record: CategoryModel) => {
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
