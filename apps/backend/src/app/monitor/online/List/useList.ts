import { useFetchOnlineList } from '@/services';
import { useMemoizedFn } from 'ahooks';
import { useListStore } from './useListStore';

export function useList() {
  const listRes = useListStore((state) => state.listRes);
  const setState = useListStore((state) => state.setState);
  const { fetchOnlineList, loading } = useFetchOnlineList();

  const fetchListPageData = useMemoizedFn(async () => {
    const res = await fetchOnlineList({});

    setState({
      listRes: res.onlines,
    });
    return res;
  });

  const refresh = useMemoizedFn(async () => {
    setState({
      refreshToken: Date.now().toString(),
    });
  });

  return {
    listRes,
    refresh,
    fetchOnlineList,
    fetchListPageData,
    loading,
  };
}