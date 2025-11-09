import { useFetchOperationLogList } from '@/services';
import { useMemoizedFn } from 'ahooks';
import { useListStore } from './useListStore';

export function useList() {
  const listRes = useListStore((state) => state.listRes);
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);
  const { fetchOperationLogList, loading } = useFetchOperationLogList();

  const fetchListPageData = useMemoizedFn(async () => {
    const res = await fetchOperationLogList({
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
    fetchOperationLogList,
    fetchListPageData,
    loading,
  };
}