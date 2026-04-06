'use client';

import type { DescribeTagListRequestParams } from "@repo/apis";
import { tagApis } from "#src/api";
import { useQuery } from '@tanstack/react-query';
import { useListStore } from './useListStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryTagList() {
  const filters = useListStore((state) => state.filters);
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['tagList'],
    queryFn: async () => {
      const params: DescribeTagListRequestParams = {
        page: pager?.page,
        pageSize: pager?.pageSize,
      };

      if (filters?.keyword) {
        params._name = filters.keyword;
      }

      const res = await tagApis.DescribeTagList(params);

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