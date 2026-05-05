'use client';

import type { ReactElement } from "react";

import { useEffect } from 'react';
import {
  Card,
  Button,
  Descriptions,
  Empty,
} from 'antd';
import {
  Icon,
} from '#src/components';
import { useListStore } from '../CacheList/useListStore';
import useDescriptions from './useDescriptions';
import { useList } from '../CacheList/useList';
import { useMemoizedFn } from 'ahooks';
import { Container } from './index.styled';

/**
 * 缓存内容详细信息
 */
export default function CacheContent(): ReactElement {
  const cacheDetailRes = useListStore(state => state.cacheDetailRes);
  const detailRefreshToken = useListStore(state => state.detailRefreshToken);
  const cacheKey = useListStore(state => state.cacheKey);
  const setState = useListStore(state => state.setState);
  const cache = cacheDetailRes?.cache;
  const [items] = useDescriptions();

  const { fetchCacheDetail } = useList();

  const handleRefresh = useMemoizedFn(() => {
    setState({
      detailRefreshToken: Date.now().toString(),
    });
  });

  useEffect(() => {
    if (detailRefreshToken) {
      fetchCacheDetail(cacheKey);
    }
  }, [detailRefreshToken]);

  return (
    <Container
      className="oic-card-w flex-1 oic-card-cache-content-w"
    >
      <Card
        title="缓存内容"
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
        <div>
          {cache ? (
            <Descriptions
              items={items}
              column={1}
            />
          ): (
            <div>
              <Empty />
            </div>
          )}
          
        </div>
      </Card>
    </Container>
  );
}