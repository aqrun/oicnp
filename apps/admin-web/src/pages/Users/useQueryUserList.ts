import {
  DescribeUserList,
  DescribeUserListRequestParams,
} from '~/api';
import { useQuery } from '@tanstack/react-query';
import { useUserStore } from './useUserStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryUserList() {
  const filters = useUserStore((state) => state.filters);
  const pager = useUserStore((state) => state.pager);
  const setState = useUserStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['userList', pager],
    queryFn: async () => {
      const params: DescribeUserListRequestParams = {
        page: pager?.page,
        page_size: pager?.pageSize,
      };

      if (filters?.keyword) {
        params.username = filters.keyword;
      }

      const res = await DescribeUserList(params);

      setState({
        pager: {
          ...pager,
          total: res?.total || 0,
        }
      });

      return res;
    }
  });

  const refresh = useMemoizedFn(() => {
    refetch();
  });

  return {
    data,
    loading: isFetching,
    refresh,
  };
}
