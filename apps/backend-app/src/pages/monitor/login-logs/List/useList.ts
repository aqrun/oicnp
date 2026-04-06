import { useFetchLoginLogList } from "#src/hooks/apis";
import { useMemoizedFn } from 'ahooks';
import { useListStore } from './useListStore';

export function useList() {
  const listRes = useListStore((state) => state.listRes);
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);
  const { fetchLoginLogList, loading } = useFetchLoginLogList();

  const fetchListPageData = useMemoizedFn(async () => {
    const res = await fetchLoginLogList({
      page: pager.page,
      pageSize: pager.pageSize,
    });

    setState({
      listRes: res,
      pager: {
        ...pager,
        total: res?.total || 0,
      },
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
    fetchLoginLogList,
    fetchListPageData,
    loading,
  };
}