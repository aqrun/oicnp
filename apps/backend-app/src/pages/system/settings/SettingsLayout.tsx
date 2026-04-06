"use client";

import { PageTitle } from "#src/components";
import type { MenuProps } from "antd";
import { Layout, Menu } from "antd";
import type { ReactElement } from "react";
import { useNavigate } from "react-router";

import { appRoutePath } from "#src/utils/app";

import { SettingsContainer } from "./index.styled";

const { Sider, Content } = Layout;

export interface SettingsLayoutProps extends React.PropsWithChildren {

}

export default function SettingsLayout({
  children,
}: SettingsLayoutProps): ReactElement {
  const navigate = useNavigate();

  const menuItems = [
    {
      key: 'message',
      label: '消息设置',
    },
    {
      key: 'account',
      label: '账号管理',
    },
    {
      key: 'profile',
      label: '个人资料',
    },
  ];

  const handleMenuSelect: MenuProps["onSelect"] = ({ key }) => {
    navigate(appRoutePath(`/system/settings/${key}`));
  };

  return (
    <SettingsContainer className="h-full">
      <PageTitle
        title='系统设置'
      />

      <div className="h-full">
        <Layout className="bg-white h-full">
          <Sider width={200} className="border-gray-200 border-r">
            <Menu
              mode="inline"
              defaultSelectedKeys={['message']}
              items={menuItems}
              className="h-full border-0"
              onSelect={handleMenuSelect}
            />
          </Sider>
          <Content className="px-6">
            {children}
          </Content>
        </Layout>
      </div>
    </SettingsContainer>
  );
}
