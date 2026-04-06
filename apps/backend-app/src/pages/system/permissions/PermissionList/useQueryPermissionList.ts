'use client';

import type { DescribePermissionListRequestParams } from '@repo/apis';
import { permissionApis } from '#src/api';
import { useQuery } from '@tanstack/react-query';
import { useListStore } from './useListStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryPermissionList() {
  const filters = useListStore((state) => state.filters);
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['permissionList'],
    queryFn: async () => {
      const params: DescribePermissionListRequestParams = {
        page: 1,
        pageSize: 1000,
      };

      if (filters?.keyword) {
        params._name = filters.keyword;
      }

      const res = await permissionApis.DescribePermissionList(params);

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
