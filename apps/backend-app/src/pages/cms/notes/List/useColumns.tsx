'use client';

import { TableProps} from 'antd';
import TableActions from './TableActions';
import { NoteModel } from "@repo/apis";
import { formatDate } from '#src/utils';

export default function useColumns() {
  const columns: TableProps<NoteModel>['columns'] = [
    {
      key: 'id',
      title: 'ID',
      dataIndex: 'id',
      width: 80,
    },
    {
      key: 'title',
      title: '标题',
      dataIndex: 'title',
    },
    {
      key: 'createdAt',
      title: '创建时间',
      dataIndex: 'createdAt',
      render: (value: string) => {
        return formatDate(value);
      }
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
      render: (value: string, record: NoteModel) => {
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