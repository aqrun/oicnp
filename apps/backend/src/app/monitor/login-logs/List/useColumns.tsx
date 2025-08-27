'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { LoginLogModel } from '@/services';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<LoginLogModel>['columns'] = [
    {
      key: 'id',
      title: 'ID',
      dataIndex: 'id',
      width: 80,
    }, 
    {
      key: 'loginAt',
      title: '登录时间',
      dataIndex: 'loginAt',
      width: 80,
    },
    {
      key: 'loginName',
      title: '登录名',
      dataIndex: 'loginName',
    },
    {
      key: 'net',
      title: '网络',
      dataIndex: 'net',
    },
    {
      key: 'ip',
      title: 'IP',
      dataIndex: 'ip',
    },
    {
      key: 'location',
      title: '位置',
      dataIndex: 'location',
    },
    {
      key: 'browser',
      title: '浏览器',
      dataIndex: 'browser',
    },
    {
      key: 'os',
      title: '操作系统',
      dataIndex: 'os',
    },
    {
      key: 'device',
      title: '设备',
      dataIndex: 'device',
    },
    {
      key: 'status',
      title: '状态',
      dataIndex: 'status',
    },
    {
      key: 'message',
      title: '消息',
      dataIndex: 'message',
    },
    {
      key: 'module',
      title: '模块',
      dataIndex: 'module',
    },
    {
      key: 'status',
      title: '状态',
      dataIndex: 'status',
    },
    {
      key: 'loginAt',
      title: '登录时间',
      dataIndex: 'loginAt',
      width: 200,
      render: (value: string) => formatDate(value),
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
      render: (value: string, record: LoginLogModel) => {
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
