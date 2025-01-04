import { STATIC_URI, BACK_BASE_URI } from '~/constants';
import { MenuItem } from '~/types';

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
  const validUri = uri?.trim()?.replace(/^\//i, '');

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
