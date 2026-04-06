'use client';

import type { DescribeCategoryListRequestParams } from "@repo/apis";
import { categoryApis } from "#src/api";
import { useQuery } from '@tanstack/react-query';
import { useListStore } from './useListStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryCategoryList() {
  const filters = useListStore((state) => state.filters);
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);

  const { isFetching, data, refetch } = useQuery({
    queryKey: ['categoryList'],
    queryFn: async () => {
      const params: DescribeCategoryListRequestParams = {
        page: pager?.page,
        pageSize: pager?.pageSize,
      };

      if (filters?.keyword) {
        params._name = filters.keyword;
      }

      const res = await categoryApis.DescribeCategoryList(params);

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