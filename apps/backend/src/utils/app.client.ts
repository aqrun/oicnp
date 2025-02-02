'use client';

import { URLHash } from './UrlHash';
import { HashState } from '@/types';

/**
 * 更新哈希参数
 */
export function setHashState(payload: HashState = {}) {
  if (typeof window === 'undefined') return;

  const {
    route,
    subRoute,
    ...params
  } = payload;

  const pathArr = [''];

  if (route) {
    pathArr.push(route);
  }

  if (subRoute) {
    pathArr.push(subRoute);
  }

  let hashStr = location.hash;
  const urlHash = new URLHash(hashStr);
  urlHash.setPathName(pathArr.join('/'))
    .addParams(params);

  location.hash = urlHash.toString();
}

/**
 * 解析哈希参数
 */
export function parseHashState(): HashState {
  if (typeof window === 'undefined') return {};

  let hashStr = location.hash;
  const urlHash = new URLHash(hashStr);

  const pathArr = urlHash.pathName?.split('/');
  const route = pathArr?.[1];
  const subRoute = pathArr?.[2];
  const params = urlHash.getParams();

  return {
    route,
    subRoute,
    ...params,
  };
}