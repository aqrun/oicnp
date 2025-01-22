'use client'

import React from 'react';
import {
  PageContainer,
  ProCard,
} from '@ant-design/pro-components';
import {
  Table,
} from 'antd';
import {
  useQuery,
} from '@tanstack/react-query';
import type { NoteItem } from '~/types';
import { fetchNoteList } from '~/api/notes';
import useColumns from './use-columns';

export default function NotesPage (): JSX.Element {
  const columns = useColumns();
  const query = useQuery({
    queryKey: ['note/list'],
    queryFn: fetchNoteList,
  });

  return (
    <PageContainer
      subTitle="日常内容小记"
    >
      <ProCard>
        <Table<NoteItem>
          columns={columns}
          dataSource={query.data?.data}
          loading={query.isLoading}
          rowKey="id"
          size="small"
          tableLayout="fixed"
        />
      </ProCard>
    </PageContainer>
  )
};
