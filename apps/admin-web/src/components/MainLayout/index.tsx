import { Outlet, useNavigate } from 'react-router';
import { Breadcrumb, Menu } from 'antd';
import { asset, r } from '~/utils';
import { CLASS_PREFIX } from '~/constants';
import cls from 'clsx';
import {
  MenuItem,
} from '~/types';
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

export interface MainLayoutProps {
  mainMenus?: MenuItem[];
  sideMenus?: MenuItem[];
  /**
   * 选中的主菜单
   */
  mainMenuKeys?: string[];
  /**
   * 选中的侧导航项
   */
  sideMenuKeys?: string[];
  /**
   * 侧导航展开项
   */
  sideMenuOpenKeys?: string[];
}

export default function MainLayout({
  mainMenus,
  sideMenus,
  mainMenuKeys,
  sideMenuKeys,
  sideMenuOpenKeys,
}: MainLayoutProps): JSX.Element {
  const navigate = useNavigate();
  const showSideNav = sideMenus && sideMenus?.length > 0;

  /**
   * 主菜单点击
   */
  const handleMainMenuSelect = useMemoizedFn((info) => {
    navigate(r(info?.key));
  });

  return (
    <Container className={cls(`${CLASS_PREFIX}-layout-container`)}>
      <Header className={cls(`${CLASS_PREFIX}-layout-header`)}>
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
            selectedKeys={mainMenuKeys}
            items={mainMenus}
            className={cls(`${CLASS_PREFIX}-header-main-menu`)}
            onSelect={handleMainMenuSelect}
          />
        )}
      </Header>
      <Main className={cls(`${CLASS_PREFIX}-layout-main`)}>
        {showSideNav && (
          <Side className={cls(`${CLASS_PREFIX}-layout-side`)}>
            <Menu
              mode="inline"
              selectedKeys={sideMenuKeys}
              openKeys={sideMenuOpenKeys}
              items={sideMenus}
              className={cls(`${CLASS_PREFIX}-side-menu`)}
              onSelect={(info) => {
                console.log('self-', info);
              }}
              onOpenChange={(openKeys: string[]) => {
                console.log('openkeys', openKeys);
              }}
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
