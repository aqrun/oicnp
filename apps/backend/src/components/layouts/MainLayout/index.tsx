'use client';

import { useMemo, useState } from 'react';
import { useRouter, usePathname } from 'next/navigation';
import { Breadcrumb, Menu } from 'antd';
import { SelectInfo } from 'rc-menu/lib/interface';
import { CLASS_PREFIX } from '@/constants';
import cls from 'clsx';
import { Icon } from '@/components';
import {
  Container,
  Header,
  Main,
  Side,
  MainContent,
  Content,
  Footer,
} from './index.styled';
import { MenuItem, BreadItem } from '@/types';
import { MenuTreeItem } from '@/services/types';
import { NavUser } from "@/components/nav-user"
import {
  SidebarFooter,
  SidebarProvider,
  SidebarMenuButton,
} from "@/components/ui/sidebar";

export interface MainLayoutProps extends React.PropsWithChildren {
  navMenus: MenuTreeItem[];
}

export function MainLayout({
  children,
  navMenus,
}: MainLayoutProps): JSX.Element {
  const router = useRouter();
  const pathname = usePathname();
  const showSideNav = true;

  const [selectedKeys, setSelectedKeys] = useState<Array<string> | undefined>(undefined);
  const [openKeys, setOpenKeys] = useState<Array<string> | undefined>(undefined);

  const sideMenus = useMemo(() => {
    const items = navMenus?.map((item) => {
      const menu: MenuItem = {
        ...(item as unknown as MenuItem),
        children: undefined,
      };

      if (item.icon) {
        menu.icon = (<Icon icon={item.icon} />);
      }

      if (item?.children?.length) {
        menu.children = item.children?.map((n) => {
          const subMenu: MenuItem = {
            ...(n as unknown as MenuItem),
            children: undefined,
          };

          if (n.icon) {
            subMenu.icon = (<Icon icon={n.icon} />);
          }

          return subMenu;
        });
      }

      return menu;
    });

    return items;
  }, [navMenus]);

  // 默认选中的菜单数据解析
  const pathnameArr = pathname?.split('/')?.filter(i => i);
  const getDefaultSelectedKeys = () => {
    if (pathnameArr?.length) {
      return [`/${pathnameArr?.slice(0, 2).join('/')}`];
    }
    return ['/dashboard'];
  }
  const defaultSelectedKeys = getDefaultSelectedKeys();
  const getDefaultOpenKeys = () => {
    if (pathnameArr?.length > 1) {
      return [`/${pathnameArr?.[0]}`];
    }
    return [];
  };
  const defaultOpenKeys = getDefaultOpenKeys();
  const getBreads = () => {
    const items: BreadItem[] = [{
      title: '首页',
      href: '/dashboard',
    }];

    let menu;
    let subMenu;

    if (pathnameArr.length) {
      menu = navMenus.find((item) => {
        return item?.path === `/${pathnameArr?.[0]}`;
      });

      if (menu) {
        items.push({
          title: menu.label,
          href: menu.path,
        });
      }

      if (menu?.children?.length && pathnameArr?.length > 1) {
        subMenu = menu?.children?.find((item) => {
          return item?.path === `/${pathnameArr?.slice(0, 2).join('/')}`;
        });

        if (subMenu) {
          items.push({
            title: subMenu.label,
            href: subMenu.path,
          });
        }
      }
    }
    
    return items;
  };
  const breads = getBreads();
  
  /**
   * 侧栏一级菜单点击
   */
  const handleOpenChange = ((paramOpenKeys: string[]) => {
    setOpenKeys(paramOpenKeys);
  });

  /**
   * 二级菜单项点击
   */
  const handleSelect = ((info: SelectInfo) => {
    setSelectedKeys(info.selectedKeys || []);
    setOpenKeys(info.keyPath?.[1] ? [info.keyPath?.[1]] : []);
    router.push(info.key);
  });

  return (
    <SidebarProvider>
      <Container className={cls(`${CLASS_PREFIX}-layout-container`)}>
        {showSideNav && (
          <Side className={cls(`${CLASS_PREFIX}-layout-side`)}>
            <Header className={cls(`${CLASS_PREFIX}-layout-header`)}>
              <SidebarMenuButton
                size="lg"
                className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
              >
                <div className="flex aspect-square size-8 items-center justify-center rounded-lg bg-sidebar-primary text-sidebar-primary-foreground">
                  <div>logo</div>
                </div>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-semibold">
                    OICNP
                  </span>
                  <span className="truncate text-xs">
                    管理系统
                  </span>
                </div>
              </SidebarMenuButton>
            </Header>
            <Menu
              mode="inline"
              defaultSelectedKeys={selectedKeys || defaultSelectedKeys}
              defaultOpenKeys={openKeys || defaultOpenKeys}
              items={sideMenus}
              className={cls(`${CLASS_PREFIX}-side-menu`)}
              onSelect={handleSelect}
              onOpenChange={handleOpenChange}
            />
            <div className={cls(`${CLASS_PREFIX}-layout-side-footer`)}>
              <SidebarFooter
                className="w-full"
              >
                <NavUser user={{
                  name: "shadcn",
                  email: "m@example.com",
                  avatar: "/avatars/shadcn.jpg",
                }} />
              </SidebarFooter>
            </div>
          </Side>
        )}
        <Main className={cls(`${CLASS_PREFIX}-layout-main`)}>
          <MainContent className={cls(`${CLASS_PREFIX}-layout-content`)}>
            <Breadcrumb
              className={cls(`${CLASS_PREFIX}-layout-bread`)}
              items={breads}
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
    </SidebarProvider>
  );
}
