import { useFetchFileList } from "#src/hooks/apis";
import { useMemoizedFn } from 'ahooks';
import { useListStore } from './useListStore';

export function useList() {
  const filesRes = useListStore((state) => state.filesRes);
  const setState = useListStore((state) => state.setState);
  const { fetchFileList, loading } = useFetchFileList();

  const fetchListPageData = useMemoizedFn(async () => {
    const res = await fetchFileList({});

    setState({
      filesRes: res?.files,
    });
    return res;
  });

  const refresh = useMemoizedFn(async () => {
    setState({
      refreshToken: Date.now().toString(),
    });
  });

  return {
    filesRes,
    refresh,
    fetchFileList,
    fetchListPageData,
    loading,
  };
}