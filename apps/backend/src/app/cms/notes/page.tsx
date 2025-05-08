'use client';

import { Empty } from 'antd';
import {
  PageTitle,
} from '@/components';
import { Container } from './index.styled';

export default function SettingsPage() {
  return (
    <Container>
      <PageTitle
        title='笔记列表'
      />

      <div>
        <Empty description="功能开发中 ^_^" />
      </div>
    </Container>
  );
}
