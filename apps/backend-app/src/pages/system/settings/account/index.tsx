'use client';

import { Button, Card } from 'antd';
import { Container } from './index.styled';

export default function AccountPage() {
  return (
    <Container className="h-full">
      <Card
        title="账号注销"
        size="small"
      >
        <div
          className="flex flex-row items-center"
        >
          <p>
            注销账号后，您将无法使用当前账号登录系统，且无法恢复。
          </p>
          <Button
            type="primary"
            danger
            size="small"
          >
            注销账号
          </Button>
        </div>
      </Card>
    </Container>
  );
}
