'use client';

import {
  Card,
  Button,
  Table,
} from 'antd';
import {
  Icon,
} from '@/components';
import useColumns from './useColumns';
import { useListStore } from '../CacheList/useListStore';
import { Container } from './index.styled';

export default function KeyList(): JSX.Element {
  const cachesRes = useListStore((state) => state.cachesRes);
  const columns = useColumns();

  return (
    <Container
      className="oic-card-w flex-1"
    >
      <Card
        title="键名列表"
        size="small"
        className="h-full"
        extra={
          <Button
            size="small"
          >
            <Icon icon="ReloadOutlined" />
          </Button>
        }
      >
        <Table
          dataSource={cachesRes?.caches || []}
          columns={columns}
          loading={false}
          rowKey="tagId"
          size="small"
          pagination={false}
        />
      </Card>
    </Container>
  );
}