import React from 'react';
import Link from 'next/link';
import { Layout, Menu, Breadcrumb } from 'antd';
import {
  Container,
  BreadWrapper,
} from './index.styled';

const { Content, Sider } = Layout;

export const LayoutMain: React.FC = ({
  children
}) => {

  return (
    <Container>
      <BreadWrapper>
        <Breadcrumb>
          <Breadcrumb.Item>Home</Breadcrumb.Item>
          <Breadcrumb.Item>List</Breadcrumb.Item>
          <Breadcrumb.Item>App</Breadcrumb.Item>
        </Breadcrumb>
      </BreadWrapper>

      <Layout>
        <Content>
          {children}
        </Content>

        <Sider className="site-layout-background">
          side
        </Sider>
      </Layout>
    </Container>
  );
};
