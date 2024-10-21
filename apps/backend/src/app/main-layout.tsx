import React, { useState } from 'react';
import Link from 'next/link';
import {
  ProConfigProvider,
  ProLayout,
} from '@ant-design/pro-components';
import type {
  ProSettings,
  MenuDataItem,
} from '@ant-design/pro-components';
import type { SiderMenuProps } from '@ant-design/pro-layout/es/components/SiderMenu/SiderMenu';
import {
  ConfigProvider,
  Dropdown,
} from 'antd';
import {
  GithubFilled,
  InfoCircleFilled,
  LogoutOutlined,
  QuestionCircleFilled,
} from '@ant-design/icons';
import { useMemoizedFn } from 'ahooks';
import { SearchInput } from '~/components';
import { useAuthState } from '~/hooks';
import layoutProps from './default-props';

export default function MainLayout ({
  children,
}: React.PropsWithChildren): JSX.Element {
  const [, setAuthState] = useAuthState();
  // eslint-disable-next-line no-unused-vars, @typescript-eslint/no-unused-vars -- desc
  const [settings, setSettings] = useState<Partial<ProSettings> | undefined>({
    fixSiderbar: true,
    layout: 'mix',
    splitMenus: false,
    siderMenuType: 'sub',
    colorPrimary: '#722ED1'
  });

  const [pathname, setPathname] = useState('/');
  // const [num, setNum] = useState(40);

  const logoutHandle = useMemoizedFn(() => {
    setAuthState({
      user: undefined,
    });
  });

  const menuItemRender = useMemoizedFn((item: MenuDataItem, dom: React.ReactNode) => {
    return (
      <Link
        href={item.path || '/'}
        onClick={() => {
          // eslint-disable-next-line no-console -- desc
          console.log('menu---', item);
          setPathname(item.path || '/welcome');
        }}
        className="oic-menu-item-render"
      >
        {dom}
      </Link>
    );
  });

  const menuFooterRender = useMemoizedFn((props?: SiderMenuProps) => {
    if (props?.collapsed) return undefined;
    return (
      <div
        style={{
          textAlign: 'center',
          paddingBlockStart: 12,
        }}
      >
        <div>© 2021 Made with love</div>
        <div>by Ant Design</div>
      </div>
    );
  });

  return (
    <ProConfigProvider hashed={false}>
      <ConfigProvider
        getTargetContainer={() => {
          return document.body;
        }}
      >
        <ProLayout
          {...layoutProps}
          actionsRender={(props) => {
            if (props.isMobile) return [];
            if (typeof window === 'undefined') return [];
            return [
              props.layout !== 'side' && document.body.clientWidth > 1400 ? (
                <SearchInput />
              ) : undefined,
              <InfoCircleFilled key="InfoCircleFilled" />,
              <QuestionCircleFilled key="QuestionCircleFilled" />,
              <GithubFilled key="GithubFilled" />,
            ];
          }}
          avatarProps={{
            src: 'https://gw.alipayobjects.com/zos/antfincdn/efFD%24IOql2/weixintupian_20170331104822.jpg',
            size: 'small',
            title: '七妮妮',
            render: (props, dom) => {
              return (
                <Dropdown
                  menu={{
                    items: [
                      {
                        key: 'logout',
                        icon: <LogoutOutlined />,
                        label: '退出登录',
                        onClick: logoutHandle,
                      },
                    ],
                  }}
                >
                  {dom}
                </Dropdown>
              );
            },
          }}
          location={{
            pathname: pathname,
          }}
          menu={{
            collapsedShowGroupTitle: true,
          }}
          menuFooterRender={menuFooterRender}
          menuItemRender={menuItemRender}
          onMenuHeaderClick={(e) => {
            // eslint-disable-next-line no-console -- desc
            console.log(e)
          }}
          {...settings}
        >
          {children}
        </ProLayout>
      </ConfigProvider>
    </ProConfigProvider>
  );
};
