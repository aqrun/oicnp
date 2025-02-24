'use client';

import type { MenuProps } from 'antd';
import type { MenuItemType } from 'antd/es/menu/interface';
import type { BreadcrumbProps } from 'antd';

export type { SubMenuType } from 'antd/es/menu/interface';

type MenuPropsItem = Omit<NonNullable<MenuProps['items']>, 'type'>[number];

/**
 * 菜单项类型
 */
export type MenuItem = MenuPropsItem & MenuItemType & {
  url?: string;
  type?: string;
  children?: MenuItem[];
  ignore?: boolean;
}

/**
 * 菜单URL项
 */
export interface RoutePathParams {
  mainMenuKey?: string;
  sideMenuOpenKey?: string;
  sideMenuKey?: string;
}

/**
 * 面包屑数据
 */
export type BreadItem = NonNullable<BreadcrumbProps['items']>[number];
