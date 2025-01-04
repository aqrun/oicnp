import { getBaseUri } from './app';
import { MenuItem } from '~/types';

/**
 * 根据当前 location path 解析对应的菜单数据
 */
export class UrlState {
  menus: MenuItem[] = [];

  pathname: string = '';

  /**
   * 解析后的合法路径参数
   */
  pathnames: string[] = [];
  /**
   * 子导航一级选中项
   */
  sideSelectedOpenKeys: string[] = [];
  /**
   *  子导航二级选中项
   */
  sideSelectedKeys: string[] = [];

  /**
   * 主菜单项
   */
  mainMenu: MenuItem | undefined = undefined;
  /**
   * 子菜单一级
   */
  sideOpenMenu : MenuItem | undefined = undefined;
  /**
   * 子菜单二级
   */
  sideMenu: MenuItem | undefined = undefined;

  /**
   * 选中的主菜单
   */
  mainMenuKey: string = '';
  /**
   * 选中的侧导航项
   */
  sideMenuKey: string = '';
  /**
   * 侧导航展开项
   */
  sideMenuOpenKey: string = '';

  constructor(pathname: string, menus: MenuItem[]) {
    /**
     * 数据重置
     */
    this.pathnames = [];
    this.mainMenu = undefined;
    this.sideOpenMenu = undefined;
    this.sideMenu = undefined;
    this.mainMenuKey = '';
    this.sideMenuKey = '';
    this.sideMenuOpenKey = '';
    this.sideSelectedKeys = [];
    this.sideSelectedOpenKeys = [];

    this.pathname = pathname;
    this.menus = menus;

    const baseUri = getBaseUri();
    this.pathnames = pathname?.replace(baseUri, '')
      .split('/')
      .filter((item) => item);

    this.getMainMenuKey();
    this.getSideMenuOpenKey();
    this.getSideMenuKey();
  }

  /**
   * 二级子菜单解析
   */
  getSideMenuKey() {
    // 当前主菜单项无子项
    if (!this.mainMenu?.children) return;

    // 子菜单一级无子项
    if (!this.sideOpenMenu?.children) {
      this.sideSelectedKeys = [`${this.sideMenuOpenKey}`];
      return;
    }

    const sideMenus = this.sideOpenMenu?.children || [];
    this.sideMenuKey = this.pathnames?.[2];

    // 没查到对应二级 使用默认的第一项
    if (!this.sideMenuKey) {
      this.sideMenuKey = `${sideMenus?.[0]?.key || ''}`;
      this.pathnames.push(this.sideMenuKey);
    }

    // 查询二级子菜单项
    this.sideMenu = sideMenus?.find((item) => {
      return item?.key === this.sideMenuKey;
    });

    this.sideSelectedOpenKeys = [this.sideMenuOpenKey];
    this.sideSelectedKeys = [`${this.sideMenuOpenKey}@${this.sideMenuKey}`];

    return this.sideMenuKey;
  }

  /**
   * 一级子菜单解析
   */
  getSideMenuOpenKey() {
    if (!this.mainMenuKey) return;

    // 当前主菜单项无子项
    if (!this.mainMenu?.children) return;

    const sideMenuOpenMenus = this.mainMenu?.children;
    this.sideMenuOpenKey = this.pathnames?.[1] || '';

    if (this.sideMenuOpenKey) {
      this.sideOpenMenu = this.mainMenu?.children?.find((item) => {
        return item?.key === this.sideMenuOpenKey;
      });
    } else {
      this.sideOpenMenu = sideMenuOpenMenus?.[0];
      this.sideMenuOpenKey = `${this.sideOpenMenu?.key || ''}`;
      this.pathnames.push(this.sideMenuOpenKey);
    }
  }

  /**
   * header 主菜单数据解析
   */
  getMainMenuKey() {
    if (!this.pathnames?.length) {
      this.mainMenu = this.menus?.[0];
      this.mainMenuKey = `${this.mainMenu?.key || ''}`;
      this.pathnames = [this.mainMenuKey];
      return this.getMainMenuKey;
    }

    const allMainMenuKeys = this.menus?.map((item) => {
      return item?.key;
    });

    this.mainMenuKey = this.pathnames?.[0];

    if (allMainMenuKeys?.includes(this.mainMenuKey)) {
      this.mainMenu = this.menus?.find((item) => {
        return item?.key === this.mainMenuKey;
      });

      return this.mainMenuKey;
    } else {
      this.mainMenu = this.menus?.[0];
      this.mainMenuKey = `${this.mainMenu?.key || ''}`;

      if (this.mainMenuKey) {
        this.pathnames = [
          this.mainMenuKey,
          ...this.pathnames,
        ];
      }
    }

    return this.mainMenuKey;
  }
}
