import React, { useState } from 'react';
import type { Metadata } from 'next'
import Link from 'next/link';
import {
  PageContainer,
  ProCard,
  ProConfigProvider,
  ProLayout,
  SettingDrawer,
} from '@ant-design/pro-components';
import type { ProSettings } from '@ant-design/pro-components';
import {
  Button,
  ConfigProvider,
  Divider,
  Dropdown,
  Input,
  Popover,
  theme,
} from 'antd';
import {
  CaretDownFilled,
  DoubleRightOutlined,
  GithubFilled,
  InfoCircleFilled,
  LogoutOutlined,
  PlusCircleFilled,
  QuestionCircleFilled,
  SearchOutlined,
} from '@ant-design/icons';
import { MenuCard, SearchInput } from '~/components';
import layoutProps from './defaultProps';
import { useMemoizedFn } from 'ahooks';
import { useRecoilState } from 'recoil';
import { useAuthState } from '~/hooks';

const MainLayout: React.FC<React.PropsWithChildren<{}>> = ({
  children,
}) => {
  const [, setAuthState] = useAuthState();
  const [settings, setSetting] = useState<Partial<ProSettings> | undefined>({
    fixSiderbar: true,
    layout: 'mix',
    splitMenus: false,
    siderMenuType: 'sub',
    colorPrimary: '#722ED1'
  });

  const [pathname, setPathname] = useState('/');
  const [num, setNum] = useState(40);

  const logoutHandle = useMemoizedFn(() => {
    setAuthState({
      user: undefined,
    });
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
          location={{
            pathname: pathname,
          }}
          menu={{
            collapsedShowGroupTitle: true,
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
          onMenuHeaderClick={(e) => console.log(e)}
          menuItemRender={(item, dom) => {
            return (
              <Link
                href={item?.path || '/'}
                onClick={() => {
                  console.log('menu---', item);
                  setPathname(item.path || '/welcome');
                }}
                className="oic-menu-item-render"
              >
                {dom}
              </Link>
            );
          }}
          menuFooterRender={(props) => {
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
          }}
          {...settings}
        >
          {children}
        </ProLayout>
      </ConfigProvider>
    </ProConfigProvider>
  );
};

export default MainLayout;
