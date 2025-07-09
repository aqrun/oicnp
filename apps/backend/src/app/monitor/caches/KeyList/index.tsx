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
import { Container } from './index.styled';

export default function KeyList(): JSX.Element {
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
          dataSource={[]}
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