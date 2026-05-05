'use client';

import type { ReactElement } from "react";

import { useEffect } from 'react';
import {
  Card,
  Button,
  Table,
} from 'antd';
import {
  Icon,
} from '#src/components';
import useColumns from './useColumns';
import { useListStore } from '../CacheList/useListStore';
import { useMemoizedFn } from 'ahooks';
import { useList } from '../CacheList/useList';
import { Container } from './index.styled';

/**
 * 缓存列表
 */
export default function KeyList(): ReactElement {
  const cachesRes = useListStore((state) => state.cachesRes);
  const scope = useListStore((state) => state.scope);
  const cacheRefreshToken = useListStore((state) => state.cacheRefreshToken);
  const setState = useListStore((state) => state.setState);
  const columns = useColumns();

  const {
    fetchCacheListByScope,
    cacheLoading
  } = useList();

  const handleRefresh = useMemoizedFn(() => {
    setState({
      cacheRefreshToken: Date.now().toString(),
      cacheDetailRes: undefined,
    });
  });

  useEffect(() => {
    if (cacheRefreshToken) {
      fetchCacheListByScope(scope);
    }
  }, [cacheRefreshToken]);

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
            onClick={handleRefresh}
            loading={cacheLoading}
          >
            <Icon icon="ReloadOutlined" />
          </Button>
        }
      >
        <Table
          dataSource={cachesRes?.caches || []}
          columns={columns}
          loading={cacheLoading}
          rowKey="id"
          size="small"
          pagination={false}
          tableLayout="auto"
        />
      </Card>
    </Container>
  );
}