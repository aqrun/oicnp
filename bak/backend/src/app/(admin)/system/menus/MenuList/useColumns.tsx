'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { MenuModel } from '@/services';
import { formatDate } from '@/utils';
import { Icon } from '@/components';

export default function useColumns() {
  const columns: TableProps<MenuModel>['columns'] = [
    {
      key: 'name',
      title: '名称',
      dataIndex: 'name',
      width: 200,
    },
    {
      key: 'icon',
      title: '图标',
      dataIndex: 'icon',
      width: 80,
      render: (value: string) => {
        return (
          <Icon
            icon={value}
          />
        );
      }
    },
    {
      key: 'weight',
      title: '排序',
      dataIndex: 'weight',
      width: 80,
    },
    {
      key: 'vid',
      title: '标识',
      dataIndex: 'vid',
      width: 200,
    },
    {
      key: 'permissions',
      title: '权限',
      dataIndex: 'permissions',
      width: 80,
    },
    {
      key: 'path',
      title: '路径',
      dataIndex: 'path',
    },
    {
      key: 'status',
      title: '状态',
      dataIndex: 'status',
      width: 120,
    },
    {
      key: 'remark',
      title: '描述',
      dataIndex: 'remark',
      width: 200,
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
      dataIndex: 'uid',
      render: (value: string) => {
        return <></>;
      }
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'roleId',
      render: (value: string, record: MenuModel) => {
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
