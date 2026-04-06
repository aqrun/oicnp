import { useFetchCronList } from "#src/hooks/apis";
import { useMemoizedFn } from 'ahooks';
import { useListStore } from './useListStore';

export function useList() {
  const listRes = useListStore((state) => state.listRes);
  const setState = useListStore((state) => state.setState);
  const { fetchCronList, loading } = useFetchCronList();

  const fetchListPageData = useMemoizedFn(async () => {
    const res = await fetchCronList({});

    setState({
      listRes: res.crons,
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
    fetchCronList,
    fetchListPageData,
    loading,
  };
}