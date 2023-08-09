'use client'

import React, { useState } from 'react';
import {
  PageContainer,
  ProCard,
} from '@ant-design/pro-components';
import {
  Button,
} from 'antd';

const TagsPage = () => {
  return (
    <PageContainer
      extra={[
        <Button key="2">操作</Button>,
        <Button
          key="1"
          type="primary"
          onClick={() => {
          }}
        >
          主操作
        </Button>,
      ]}
      subTitle="简单的描述"
      footer={[
        <Button key="3">重置</Button>,
        <Button key="2" type="primary">
          提交
        </Button>,
      ]}
    >
      <ProCard
        style={{
          height: '200vh',
          minHeight: 800,
        }}
      >
        <div>
          page tags
        </div>
      </ProCard>
    </PageContainer>
  )
};

export default TagsPage;
