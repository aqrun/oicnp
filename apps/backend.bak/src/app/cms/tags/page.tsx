'use client'

import React from 'react';
import {
  PageContainer,
  ProCard,
} from '@ant-design/pro-components';
import {
  Button,
} from 'antd';

export default function TagsPage (): JSX.Element {
  return (
    <PageContainer
      extra={[
        <Button key="2">操作</Button>,
        <Button
          key="1"
          onClick={function test() { return 1 }}
          type="default"
        >
          主操作
        </Button>,
      ]}
      footer={[
        <Button key="3">重置</Button>,
        <Button key="2" type="primary">
          提交
        </Button>,
      ]}
      subTitle="简单的描述"
    >
      <ProCard>
        <div>
          page tags
        </div>
      </ProCard>
    </PageContainer>
  )
};
