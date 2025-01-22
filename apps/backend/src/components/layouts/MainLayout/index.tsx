'use client';

import { Breadcrumb, Menu, Flex } from 'antd';
import { asset, r } from '@/utils';
import { CLASS_PREFIX } from '@/constants';
import cls from 'clsx';
import HeaderUser from '../HeaderUser';
import NoAuth from '../NoAuth';
import { useMemoizedFn } from 'ahooks';
import {
  Container,
  Header,
  Main,
  Side,
  MainContent,
  Content,
  Footer,
} from './index.styled';
import { MenuItem } from '@/types';

export interface MainLayoutProps extends React.PropsWithChildren {

}

export function MainLayout({
  children,
}: MainLayoutProps): JSX.Element {
  const showSideNav = true;
  const mainMenus: MenuItem[] = [];

  /**
   * 主菜单点击
   */
  const handleMainMenuSelect = ((info) => {
    // const mainMenu = menus?.find((item) => {
    //   return item.key === info?.key;
    // });
    // const validKey = mainMenu?.ignore ? '' : info?.key;
    // setState({
    //   sideMenuOpenKeys: undefined,
    //   sideMenuKeys: undefined,
    // });
    // navigate(r(validKey));
  });
  
  /**
   * 侧栏一级菜单点击
   */
  const handleOpenChange = ((openKeys: string[]) => {
    // setState({
    //   sideMenuOpenKeys: openKeys,
    // });
  });

  /**
   * 二级菜单项点击
   */
  const handleSelect = ((info) => {
    // const keyPath: string[] = [`${urlState?.mainMenuKey || ''}`];

    // if (info?.keyPath?.length === 2) {
    //   const menuKey = (info?.keyPath?.[0] || '')?.split('@')?.[1];

    //   keyPath.push(info?.keyPath?.[1]);
    //   keyPath.push(menuKey);
    // } else if (info?.keyPath?.length === 1) {
    //   keyPath.push(info?.key);
    // }

    // const uri = getRoutePathByKeyPath(menus || [], keyPath);
    // navigate(r(uri));

    // setState({
    //   sideMenuKeys: info?.selectedKeys || [],
    // });
  });

  const noAuth = false;
  let content: React.ReactNode = '';

  if (noAuth) {
    content = <NoAuth />
  }

  return (
    <Container className={cls(`${CLASS_PREFIX}-layout-container`)}>
      <Header className={cls(`${CLASS_PREFIX}-layout-header`)}>
        <Flex align="center">
          <div className={cls(`${CLASS_PREFIX}-logo`)}>
            <a
              href={r('/')}
            >
              <img alt="logo" src={asset('react.svg')} />
            </a>
          </div>
          {mainMenus && mainMenus?.length > 0 && (
            <Menu
              theme="dark"
              mode="horizontal"
              selectedKeys={['']}
              items={mainMenus}
              className={cls(`${CLASS_PREFIX}-header-main-menu`)}
              onSelect={handleMainMenuSelect}
            />
          )}
        </Flex>
        <Flex align="center">
          <HeaderUser />
        </Flex>
      </Header>
      <Main className={cls(`${CLASS_PREFIX}-layout-main`)}>
        {showSideNav && (
          <Side className={cls(`${CLASS_PREFIX}-layout-side`)}>
            <Menu
              mode="inline"
              // selectedKeys={selectedKeys}
              // openKeys={openKeys}
              // items={sideMenus}
              className={cls(`${CLASS_PREFIX}-side-menu`)}
              onSelect={handleSelect}
              onOpenChange={handleOpenChange}
            />
          </Side>
        )}
        <MainContent className={cls(`${CLASS_PREFIX}-layout-content`)}>
          <Breadcrumb
            className={cls(`${CLASS_PREFIX}-layout-bread`)}
            // items={breads}
          />
          <Content className={cls(`${CLASS_PREFIX}-layout-content`)}>
            {children}
          </Content>
          <Footer className={cls(`${CLASS_PREFIX}-layout-footer`)}>
            OICNP Admin ©{new Date().getFullYear()} Created by AQRun & ❤️ 
          </Footer>
        </MainContent>
      </Main>
    </Container>
  );
}
