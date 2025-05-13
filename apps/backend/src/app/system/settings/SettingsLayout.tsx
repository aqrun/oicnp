'use client';

import { Layout, Menu } from 'antd';
import {
  PageTitle,
} from '@/components';
import { SelectInfo } from 'rc-menu/lib/interface';
import { useRouter } from 'next/navigation';
import { r } from '@/utils';
import { SettingsContainer } from './index.styled';

const { Sider, Content } = Layout;

export interface SettingsLayoutProps extends React.PropsWithChildren {

}

export default function SettingsLayout({
  children,
}: SettingsLayoutProps): JSX.Element {
  const router = useRouter();

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

  const handleMenuSelect = (info: SelectInfo) => {
    router.push(r(`/system/settings/${info.key}`));
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
