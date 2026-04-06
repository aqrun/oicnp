import { useFetchPositionList } from '#src/hooks/apis';
import { useMemoizedFn } from 'ahooks';
import { useListStore } from './useListStore';

export function useList() {
  const listRes = useListStore((state) => state.listRes);
  const setState = useListStore((state) => state.setState);
  const { fetchPositionList, loading } = useFetchPositionList();

  const fetchListPageData = useMemoizedFn(async () => {
    const res = await fetchPositionList({});

    setState({
      listRes: res,
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
    fetchPositionList,
    fetchListPageData,
    loading,
  };
}