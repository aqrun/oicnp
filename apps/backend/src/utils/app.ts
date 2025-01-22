import { STATIC_URI, BACK_BASE_URI } from '@/constants';
import { MenuItem, BreadItem } from '@/types';
import { UrlState } from './UrlState';

/**
 * 获取链接前缀
 */
export function getBaseUri(isStatic = false) {
  let baseUri = BACK_BASE_URI;

  if (isStatic) {
    baseUri = STATIC_URI;
  }

  // 移除尾部的 /
  const validBaseUri = baseUri?.trim()?.replace(/\/$/i, '');

  return validBaseUri;
}

/**
 * 统一链接处理
 * 
 * @param isStatic 是否静态资源
 */
export function r(uri: string, isStatic = false) {
  const baseUri = getBaseUri(isStatic);

  if (!uri?.trim()) return baseUri;

  // 移除开头的 / 
  const validUri = uri?.trim()
    ?.replace(/^\//i, '');

  // uri 为空 直接返回
  if (!validUri) {
    return baseUri;
  }

  return `${baseUri}/${validUri}`;
}

/**
 * 获取静态资源链接
 */
export function asset(uri: string): string {
  return r(uri, true);
}

/**
 * 根据 key 生成路由URI
 */
export function getRoutePathByKeyPath(menus: MenuItem[], keyPath: string[]) {
  const mainMenu = menus?.find((item) => {
    return item?.key === keyPath?.[0];
  });

  let sideOpenMenu;
  let sideMenu;

  if (keyPath?.[1]) {
    sideOpenMenu = mainMenu?.children?.find((item) => {
      return item?.key === keyPath?.[1];
    });
  }

  if (keyPath?.[2]) {    
    sideMenu = sideOpenMenu?.children?.find((item) => {
      return item?.key === keyPath?.[2];
    });
  }

  const keys: string[] = [];

  if (mainMenu && !mainMenu?.ignore) {
    keys.push(`${mainMenu.key || ''}`);
  }

  if (sideOpenMenu && !sideOpenMenu?.ignore) {
    keys.push(`${sideOpenMenu.key || ''}`);
  }

  if (sideMenu && !sideMenu?.ignore) {
    keys.push(`${sideMenu.key || ''}`);
  }

  if (keys?.length) {
    return `/${keys.join('/')}`;
  }

  return '/';
}

/**
 * 生成页面标题
 */
export function getPageTitle(urlState: UrlState) {
  const siteName = 'OICNP 管理系统';

  if (!urlState) return siteName;

  const names = [];

  if (urlState?.sideOpenMenu) {
    names.push(urlState?.sideOpenMenu?.label);
  }

  if (urlState?.sideMenu) {
    names.push(urlState?.sideMenu?.label);
  }

  if (names?.length) {
    names.push('-');
  }

  if (urlState?.mainMenu) {
    names.push(urlState?.mainMenu?.label);
  }

  return `${names.join('')} - ${siteName}`;
}

/**
 * 生成面包屑列表
 */
export function getBreadItems(menus: MenuItem[], urlState?: UrlState) {
  if (!urlState) return [];

  const items: BreadItem[] = [];

  if (!urlState?.mainMenu) return items;

  const uri1 = getRoutePathByKeyPath(menus, [`${urlState?.mainMenu?.key || ''}`]);

  items.push({
    title: `${urlState?.mainMenu?.label || ''}`,
    href: r(uri1),
  });

  if (!urlState?.sideOpenMenu) return [];

  let uri2 = '';

  // 存在第三项才生成
  if (urlState?.sideMenu) {
    uri2 = getRoutePathByKeyPath(menus, [
      `${urlState?.mainMenu?.key || ''}`,
      `${urlState?.sideOpenMenu?.key || ''}`,
    ]);
  }

  items.push({
    title: `${urlState?.sideOpenMenu?.label || ''}`,
    href: r(uri2),
  });

  if (!urlState?.sideMenu) return items;

  items.push({
    title: `${urlState?.sideMenu?.label || ''}`,
    href: '',
  });

  return items;
}
