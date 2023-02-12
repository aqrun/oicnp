import React from 'react';
import Link from 'next/link';
import { Layout, Menu, Breadcrumb as BaseBreadCrumb } from 'antd';
import {
  Container,
  BreadWrapper,
} from './index.styled';

const { Content, Sider } = Layout;

const Breadcrumb = BaseBreadCrumb as any;

export interface LayoutMainProps {
  children?: React.ReactNode;
}

export const LayoutMain: React.FC<LayoutMainProps> = ({
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
