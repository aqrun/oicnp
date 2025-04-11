'use client';

import {
  DescribePermissionList,
  DescribePermissionListRequestParams,
} from '@/services';
import { useQuery } from '@tanstack/react-query';
import { usePermissionStore } from './usePermissionStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryRoleList() {
  const filters = usePermissionStore((state) => state.filters);
  const pager = usePermissionStore((state) => state.pager);
  const setState = usePermissionStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['userList'],
    queryFn: async () => {
      const params: DescribePermissionListRequestParams = {
        page: 1,
        pageSize: 1000,
      };

      if (filters?.keyword) {
        params._name = filters.keyword;
      }

      const res = await DescribePermissionList(params);

      setState({
        pager: {
          ...pager,
          page: res?.page,
          pageSize: res?.pageSize,
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
