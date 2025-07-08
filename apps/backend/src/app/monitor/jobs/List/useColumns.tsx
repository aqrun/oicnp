'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { CronModel } from '@/services';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<CronModel>['columns'] = [
    {
      key: 'id',
      title: 'ID',
      dataIndex: 'id',
      width: 80,
    }, 
    {
      key: 'vid',
      title: 'Token ID',
      dataIndex: 'vid',
      width: 80,
    },
    {
      key: 'name',
      title: '任务名称',
      dataIndex: 'name',
    },
    {
      key: 'group',
      title: '任务组',
      dataIndex: 'group',
      width: 80,
    },
    {
      key: 'invokeTarget',
      title: '执行目标',
      dataIndex: 'invokeTarget',
    },
    {
      key: 'expression',
      title: '执行表达式',
      dataIndex: 'expression',
    },
    {
      key: 'misfirePolicy',
      title: '错失策略',
      dataIndex: 'misfirePolicy',
    },
    {
      key: 'concurrent',
      title: '并发',
      dataIndex: 'concurrent',
    },
    {
      key: 'status',
      title: '状态',
      dataIndex: 'status',
    },
    {
      key: 'lastTime',
      title: '上次执行时间',
      dataIndex: 'lastTime',
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
      render: (value: string, record: CronModel) => {
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
