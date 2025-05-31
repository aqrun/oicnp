'use client';

import {
  DescribeNodeList,
  DescribeNodeListRequestParams,
} from '@/services';
import { useQuery } from '@tanstack/react-query';
import { useListStore } from './useListStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryNodeList() {
  const filters = useListStore((state) => state.filters);
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['nodeList'],
    queryFn: async () => {
      const params: DescribeNodeListRequestParams = {
        page: pager?.page,
        pageSize: pager?.pageSize,
      };

      if (filters?.keyword) {
        params._name = filters.keyword;
      }

      const res = await DescribeNodeList(params);

      setState({
        pager: {
          ...pager,
          total: res?.total || 0,
        }
      });

      return res;
    },
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
