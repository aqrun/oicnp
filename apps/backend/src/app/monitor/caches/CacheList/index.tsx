'use client';

import { useEffect } from 'react';
import {
  Card,
  Button,
  Table,
} from 'antd';
import {
  Icon,
} from '@/components';
import useColumns from './useColumns';
import { Container } from './index.styled';
import { useList } from './useList';
import { useMemoizedFn } from 'ahooks';

export default function CacheList(): JSX.Element {
  const {
    scopesRes,
    refresh,
    loading,
    fetchListPageData,
    refreshToken,
  } = useList();
  const columns = useColumns();

  const handleRefresh = useMemoizedFn(() => {
    refresh();
  });

  useEffect(() => {
    if (refreshToken) {
      fetchListPageData();
    }
  }, [refreshToken]);
  useEffect(() => {
    fetchListPageData();
  }, []);

  return (
    <Container
      className="oic-card-w flex-1"
    >
      <Card
        title="缓存列表"
        size="small"
        className="h-full"
        extra={
          <Button
            size="small"
            onClick={handleRefresh}
          >
            <Icon icon="ReloadOutlined" />
          </Button>
        }
      >
        <Table
          dataSource={scopesRes?.scopes || []}
          columns={columns}
          loading={loading}
          rowKey="scope"
          size="small"
          pagination={false}
        />
      </Card>
    </Container>
  );
}