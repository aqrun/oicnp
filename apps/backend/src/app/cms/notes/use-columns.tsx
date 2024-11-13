import type { TableProps } from 'antd';
import type { NoteItem } from '~/types';
import TableActions from './table-actions';
import { dateFormat } from '~/utils';

type Columns = TableProps<NoteItem>['columns'];

export default function useColumns(): Columns {

  function getColumns(): Columns {
    return [
      {
        title: 'ID',
        dataIndex: 'id',
        key: 'id',
        width: 60,
      },
      {
        title: '标题',
        dataIndex: 'title',
        key: 'title',
        width: 400,
      },
      {
        title: '内容',
        dataIndex: 'content',
        key: 'content',
      },
      {
        title: '创建时间',
        dataIndex: 'created_at',
        key: 'created_at',
        width: 220,
        render: (val: string) => {
          return dateFormat(val);
        }
      },
      {
        title: '操作',
        dataIndex: 'id',
        key: 'action',
        width: 300,
        render: (_, record) => {
          return (
            <TableActions
              record={record}
            />
          );
        },
      },
    ];
  }
  const columns = getColumns();

  return columns;
}