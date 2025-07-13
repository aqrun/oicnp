'use client';

import {
  Card,
  Button,
  Descriptions,
  Empty,
} from 'antd';
import {
  Icon,
} from '@/components';
import { useListStore } from '../CacheList/useListStore';
import useDescriptions from './useDescriptions';
import { Container } from './index.styled';

export default function CacheContent(): JSX.Element {
  const cacheDetailRes = useListStore(state => state.cacheDetailRes);
  const cache = cacheDetailRes?.cache;
  const [items] = useDescriptions();

  return (
    <Container
      className="oic-card-w flex-1"
    >
      <Card
        title="缓存内容"
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