'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import {
  DescribeCacheList,
  DescribeCacheScopeList,
} from './client';
import {
  DescribeCacheListRequestParams,
  DescribeCacheScopeListRequestParams,
} from './types';

/**
 * 获取缓存列表
 */
export function useFetchCacheList() {
  const [loading, setLoading] = useState(false);

  const fetchCacheList = useMemoizedFn(async (params: DescribeCacheListRequestParams) => {
    setLoading(true);
    const res = await DescribeCacheList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
      ...(params || {}),
    });
    setLoading(false);
    return res;
  });

  return {
    loading,
    fetchCacheList,
  };
}

export function useFetchCacheScopeList() {
  const [loading, setLoading] = useState(false);

  const fetchCacheScopeList = useMemoizedFn(async (params: DescribeCacheScopeListRequestParams = {}) => {
    setLoading(true);
    const res = await DescribeCacheScopeList(params);
    setLoading(false);
    return res;
  });

  return {
    loading,
    fetchCacheScopeList,
  };
}
