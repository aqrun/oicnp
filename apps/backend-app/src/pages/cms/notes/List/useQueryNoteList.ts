'use client';

import type { DescribeNoteListRequestParams } from "@repo/apis";
import { noteApis } from "#src/api";
import { useQuery } from '@tanstack/react-query';
import { useListStore } from './useListStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryNoteList() {
  const filters = useListStore((state) => state.filters);
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['noteList'],
    queryFn: async () => {
      const params: DescribeNoteListRequestParams = {
        page: pager?.page,
        pageSize: pager?.pageSize,
      };

      if (filters?.keyword) {
        params._name = filters.keyword;
      }

      const res = await noteApis.DescribeNoteList(params);

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