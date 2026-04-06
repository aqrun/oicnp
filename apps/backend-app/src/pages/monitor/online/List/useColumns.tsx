'use client';

import { TableProps} from 'antd';
import TableActions from './TableActions';
import { OnlineModel } from "@repo/apis";
import { formatDate } from '#src/utils';

export default function useColumns() {
  const columns: TableProps<OnlineModel>['columns'] = [
    {
      key: 'uid',
      title: 'ID',
      dataIndex: 'uid',
      width: 80,
    }, 
    {
      key: 'tokenId',
      title: 'Token ID',
      dataIndex: 'tokenId',
      width: 80,
    },
    {
      key: 'username',
      title: '用户名',
      dataIndex: 'username',
    },
    {
      key: 'dptName',
      title: '部门',
      dataIndex: 'dptName',
      width: 80,
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
      key: 'device',
      title: '设备',
      dataIndex: 'device',
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
      key: 'loginAt',
      title: '登录时间',
      dataIndex: 'loginAt',
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
      render: (value: string, record: OnlineModel) => {
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