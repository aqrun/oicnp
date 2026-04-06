'use client';

import { TableProps} from 'antd';
import TableActions from './TableActions';
import { TagModel } from "@repo/apis";

export default function useColumns() {
  const columns: TableProps<TagModel>['columns'] = [
    {
      key: 'tagId',
      title: 'ID',
      dataIndex: 'tagId',
      width: 80,
    },
    {
      key: 'tagVid',
      title: '标识',
      dataIndex: 'tagVid',
      width: 120,
    },
    {
      key: 'tagName',
      title: '标签名称',
      dataIndex: 'tagName',
    },
    {
      key: 'count',
      title: '计数',
      dataIndex: 'tagCount',
      width: 80,
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
      dataIndex: 'tagId',
      render: () => {
        return <></>;
      }
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'tagId',
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