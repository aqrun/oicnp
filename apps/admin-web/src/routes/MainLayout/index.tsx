import { useEffect } from 'react';
import { Outlet, useNavigate } from 'react-router';
import { Breadcrumb, Menu, Flex } from 'antd';
import { asset, r, getRoutePathByKeyPath } from '~/utils';
import { CLASS_PREFIX } from '~/constants';
import { useAppStore } from '~/stores';
import cls from 'clsx';
import useMenus from '../useMenus';
import { useMemoizedFn } from 'ahooks';
import { useGlobalState } from '~/context';
import HeaderUser from './HeaderUser';
import {
  Container,
  Header,
  Main,
  Side,
  MainContent,
  Content,
  Footer,
} from './index.styled';

export default function MainLayout(): JSX.Element {
  const navigate = useNavigate();
  const { urlState, menus, authState } = useGlobalState();
  const setState = useAppStore((state) => state.setState);
  const sideMenuOpenKeys = useAppStore((state) => state.sideMenuOpenKeys);
  const sideMenuKeys = useAppStore((state) => state.sideMenuKeys);
  
  const { mainMenus, sideMenus } = useMenus();
  const showSideNav = sideMenus && sideMenus?.length > 0;

  // 展开的菜单项
  let openKeys = urlState?.sideSelectedOpenKeys;
  let selectedKeys: string[] = urlState?.sideSelectedKeys || [];

  if (typeof sideMenuOpenKeys !== 'undefined') {
    openKeys = sideMenuOpenKeys;
  }

  if (typeof sideMenuKeys !== 'undefined') {
    selectedKeys = sideMenuKeys;
  }

  /**
   * 主菜单点击
   */
  const handleMainMenuSelect = useMemoizedFn((info) => {
    const mainMenu = menus?.find((item) => {
      return item.key === info?.key;
    });
    const validKey = mainMenu?.ignore ? '' : info?.key;
    navigate(r(validKey));
  });
  
  /**
   * 侧栏一级菜单点击
   */
  const handleOpenChange = useMemoizedFn((openKeys: string[]) => {
    setState({
      sideMenuOpenKeys: openKeys,
    });
  });

  /**
   * 二级菜单项点击
   */
  const handleSelect = useMemoizedFn((info) => {
    const keyPath: string[] = [`${urlState?.mainMenuKey || ''}`];

    if (info?.keyPath?.length === 2) {
      const menuKey = (info?.keyPath?.[0] || '')?.split('@')?.[1];

      keyPath.push(info?.keyPath?.[1]);
      keyPath.push(menuKey);
    } else if (info?.keyPath?.length === 1) {
      keyPath.push(info?.key);
    }

    const uri = getRoutePathByKeyPath(menus || [], keyPath);
    navigate(r(uri));

    setState({
      sideMenuKeys: info?.selectedKeys || [],
    });
  });

  useEffect(() => {
    if (!authState) {
      setState({
        sideMenuOpenKeys: undefined,
        sideMenuKeys: undefined,
      });
      navigate(r('/login'));
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [authState]);

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
              selectedKeys={[urlState?.mainMenuKey || '']}
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
              selectedKeys={selectedKeys}
              openKeys={openKeys}
              items={sideMenus}
              className={cls(`${CLASS_PREFIX}-side-menu`)}
              onSelect={handleSelect}
              onOpenChange={handleOpenChange}
            />
          </Side>
        )}
        <MainContent className={cls(`${CLASS_PREFIX}-layout-content`)}>
          <Breadcrumb className={cls(`${CLASS_PREFIX}-layout-bread`)}>
            <Breadcrumb.Item>Home</Breadcrumb.Item>
            <Breadcrumb.Item>List</Breadcrumb.Item>
            <Breadcrumb.Item>App</Breadcrumb.Item>
          </Breadcrumb>
          <Content className={cls(`${CLASS_PREFIX}-layout-content`)}>
            <Outlet />
          </Content>
          <Footer className={cls(`${CLASS_PREFIX}-layout-footer`)}>
            Ant Design ©{new Date().getFullYear()} Created by Ant UED
          </Footer>
        </MainContent>
      </Main>
    </Container>
  );
}
