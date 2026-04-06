'use client';

import { Image } from 'antd';
import type { TableProps} from 'antd';
import TableActions from './TableActions';
import type { UploadFileRes } from "@repo/apis";
import { formatDate } from '#src/utils';

export default function useColumns() {
  const columns: TableProps<UploadFileRes>['columns'] = [
    {
      key: 'fileId',
      title: 'ID',
      dataIndex: 'id',
      width: 80,
    },
    {
      key: 'preview',
      title: '预览',
      dataIndex: 'url',
      width: 80,
      render: (value: string, record: UploadFileRes) => {
        return (
          <Image
            src={record?.link || record?.url}
            alt={record?.name}
            width={60}
            height={60}
            preview={false}
          />
        );
      }
    },
    {
      key: 'filename',
      title: '文件名',
      dataIndex: 'name',
      width: 200,
    },
    {
      key: 'uri',
      title: '文件路径',
      dataIndex: 'uri',
      width: 80,
    },
    {
      key: 'storage',
      title: '存储',
      dataIndex: 'storage',
      width: 200,
      render: (value: string) => value,
    },
    {
      key: 'createdAt',
      title: '创建时间',
      dataIndex: 'createdAt',
      width: 200,
      render: (value: string) => formatDate(value),
    },
    {
      key: 'empty',
      title: null,
      dataIndex: 'nid',
      render: () => <></>,
    },
    {
      key: 'action',
      title: '操作',
      fixed: 'right',
      width: 200,
      dataIndex: 'nid',
      render: (value: string, record: UploadFileRes) => (
        <TableActions record={record} />
      )
    }
  ];

  return columns;
}
