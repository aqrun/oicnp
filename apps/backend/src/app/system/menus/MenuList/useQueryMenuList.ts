'use client';

import {
  DescribeMenuList,
  DescribeMenuListRequestParams,
} from '@/services';
import { useQuery } from '@tanstack/react-query';
import { useMenuStore } from './useMenuStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryMenuList() {
  const filters = useMenuStore((state) => state.filters);
  const pager = useMenuStore((state) => state.pager);
  const setState = useMenuStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['menuList'],
    queryFn: async () => {
      const params: DescribeMenuListRequestParams = {
        page: pager?.page,
        pageSize: pager?.pageSize,
      };

      if (filters?.keyword) {
        params._name = filters.keyword;
      }

      const res = await DescribeMenuList(params);

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
