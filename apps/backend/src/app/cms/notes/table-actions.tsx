import React from 'react';
import { Space, Button } from 'antd';
import Link from 'next/link';
import type { NoteItem } from '~/types';

export interface TableActionsProps {
  record?: NoteItem;
}

export default function TableActions({
  record,
}: TableActionsProps): JSX.Element {

  return (
    <Space
      size="middle"
    >
      <Link
        href={`${record?.id}`}
      >
        查看
      </Link>
      <Link
        href=""
      >
        编辑
      </Link>
      <Button
        danger
        type="text"
      >
        删除
      </Button>
    </Space>
  );
}
