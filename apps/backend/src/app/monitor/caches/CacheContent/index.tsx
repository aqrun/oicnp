'use client';

import {
  Card,
  Button,
} from 'antd';
import {
  Icon,
} from '@/components';
import { Container } from './index.styled';

export default function CacheContent(): JSX.Element {
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

      </Card>
    </Container>
  );
}