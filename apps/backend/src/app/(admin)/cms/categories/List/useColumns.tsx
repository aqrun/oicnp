'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { CategoryModel } from '@/services';

export default function useColumns() {
  const columns: TableProps<CategoryModel>['columns'] = [
    {
      key: 'catName',
      title: '分类名称',
      dataIndex: 'catName',
      width: 200,
    },
    {
      key: 'catVid',
      title: '标识',
      dataIndex: 'catVid',
      width: 200,
    },
    {
      key: 'weight',
      title: '权重',
      dataIndex: 'weight',
      width: 80,
    },
    {
      key: 'catDesc',
      title: '分类描述',
      dataIndex: 'catDesc',
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
