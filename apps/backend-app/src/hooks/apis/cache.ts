import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { cacheApis } from '#src/api';
import {
  DescribeCacheListRequestParams,
  DescribeCacheScopeListRequestParams,
} from '@repo/apis';

/**
 * 获取缓存列表
 */
export function useFetchCacheList() {
  const [loading, setLoading] = useState(false);

  const fetchCacheList = useMemoizedFn(async (params: DescribeCacheListRequestParams) => {
    setLoading(true);
    const res = await cacheApis.DescribeCacheList({
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
    const res = await cacheApis.DescribeCacheScopeList(params);
    setLoading(false);
    return res;
  });

  return {
    loading,
    fetchCacheScopeList,
  };
}
