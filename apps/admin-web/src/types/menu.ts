import type { MenuProps } from 'antd';
import type { MenuItemType } from 'antd/es/menu/interface';

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
