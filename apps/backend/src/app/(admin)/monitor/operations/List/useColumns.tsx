'use client';

import type { TableProps} from 'antd';
import TableActions from './TableActions';
import { OperationLogModel } from '@/services';
import { formatDate } from '@/utils';

export default function useColumns() {
  const columns: TableProps<OperationLogModel>['columns'] = [
    {
      key: 'id',
      title: 'ID',
      dataIndex: 'id',
      width: 80,
    }, 
    {
      key: 'timeId',
      title: '时间ID',
      dataIndex: 'timeId',
      width: 80,
    },
    {
      key: 'title',
      title: '标题',
      dataIndex: 'title',
    },
    {
      key: 'businessType',
      title: '业务类型',
      dataIndex: 'businessType',
    },
    {
      key: 'requestMethod',
      title: '请求方式',
      dataIndex: 'requestMethod',
    },
    {
      key: 'operatorType',
      title: '操作类型',
      dataIndex: 'operatorType',
    },
    {
      key: 'name',
      title: '操作人',
      dataIndex: 'name',
    },
    {
      key: 'departmentName',
      title: '部门名称',
      dataIndex: 'departmentName',
    },
    {
      key: 'ip',
      title: 'IP',
      dataIndex: 'ip',
    },
    {
      key: 'url',
      title: '请求URL',
      dataIndex: 'url',
    },
    {
      key: 'location',
      title: '位置',
      dataIndex: 'location',
    },
    {
      key: 'param',
      title: '参数',
      dataIndex: 'param',
    },
    {
      key: 'pathParam',
      title: '路径参数',
      dataIndex: 'pathParam',
    },
    
    {
      key: 'status',
      title: '状态',
      dataIndex: 'status',
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
      render: (value: string, record: OperationLogModel) => {
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
