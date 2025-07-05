import { useFetchFileList } from '@/services';
import { useMemoizedFn } from 'ahooks';

export function useList() {
  const { fetchFileList, loading } = useFetchFileList();

  const refresh = useMemoizedFn(async () => {
    const res = await fetchFileList({});
    return res;
  });

  return {
    refresh,
    fetchFileList,
    loading,
  };
}