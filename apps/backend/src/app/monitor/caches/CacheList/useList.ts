import {
  useFetchCacheScopeList,
  useFetchCacheList,
  DescribeCacheDetail,
} from '@/services';
import { useMemoizedFn } from 'ahooks';
import { useListStore } from './useListStore';

export function useList() {
  const cachesRes = useListStore((state) => state.cachesRes);
  const scopesRes = useListStore((state) => state.scopesRes);
  const refreshToken = useListStore((state) => state.refreshToken);
  const setState = useListStore((state) => state.setState);
  const { fetchCacheScopeList, loading } = useFetchCacheScopeList();
  const {
    fetchCacheList,
    loading: cacheLoading,
  } = useFetchCacheList();

  const fetchListPageData = useMemoizedFn(async () => {
    const res = await fetchCacheScopeList({});

    setState({
      scopesRes: res,
    });
    return res;
  });

  const fetchCacheListByScope = useMemoizedFn(async (paramScope: string) => {
    const res = await fetchCacheList({
      scope: paramScope,
    });

    setState({
      scope: paramScope,
      cachesRes: res,
    });
    return res;
  });

  const fetchCacheDetail = useMemoizedFn(async (cacheKey: string) => {
    const res = await DescribeCacheDetail({
      cacheKey,
    });

    setState({
      cacheDetailRes: res,
      cacheKey
    });
    return res;
  });

  const refresh = useMemoizedFn(async () => {
    setState({
      refreshToken: Date.now().toString(),
      cachesRes: undefined,
      cacheDetailRes: undefined,
    });
  });

  return {
    scopesRes,
    cachesRes,
    refresh,
    fetchListPageData,
    fetchCacheListByScope,
    refreshToken,
    loading,
    cacheLoading,
    fetchCacheDetail,
  };
}