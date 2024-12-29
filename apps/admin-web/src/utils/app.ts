import { STATIC_URI, BACK_BASE_URI } from '~/constants';

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
  if (!uri?.trim()) return uri;

  const baseUri = getBaseUri(isStatic);
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
