'use client';

import {
  DescribeRoleList,
  DescribeRoleListRequestParams,
} from '@/services';
import { useQuery } from '@tanstack/react-query';
import { useRoleStore } from './useRoleStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryRoleList() {
  const filters = useRoleStore((state) => state.filters);
  const pager = useRoleStore((state) => state.pager);
  const setState = useRoleStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['userList'],
    queryFn: async () => {
      const params: DescribeRoleListRequestParams = {
        page: pager?.page,
        page_size: pager?.pageSize,
      };

      if (filters?.keyword) {
        params._name = filters.keyword;
      }

      const res = await DescribeRoleList(params);

      setState({
        pager: {
          ...pager,
          total: res?.total || 0,
        }
      });

      return res;
    },
  },);

  const refresh = useMemoizedFn(() => {
    refetch();
  });

  return {
    data,
    loading: isFetching,
    refresh,
  };
}
