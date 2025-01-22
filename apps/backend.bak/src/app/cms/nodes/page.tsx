'use client'

import React from 'react';
import {
  PageContainer,
  ProCard,
} from '@ant-design/pro-components';
import {
  Button,
} from 'antd';

export default function NodesPage (): JSX.Element {
  return (
    <PageContainer
      extra={[
        <Button key="2">操作</Button>,
        <Button
          key="1"
          type="primary"
          // onClick={() => {
          // }}
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
      subTitle="简单的描述nodes"
    >
      <ProCard
        style={{
          height: '200vh',
          minHeight: 800,
        }}
      >
        <div>
          page nodes
        </div>
      </ProCard>
    </PageContainer>
  )
}
