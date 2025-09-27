'use client';

import { useState } from 'react';
import {
  DescribeNodeList,
  DescribeNodeListRequestParams,
  DescribeMultiNodesCategories,
  DescribeMultiNodesCategoriesRequestParams,
} from '@/services';
import { useListStore } from './useListStore';
import { useMemoizedFn } from 'ahooks';

export function useQueryNodeList() {
  const filters = useListStore((state) => state.filters);
  const pager = useListStore((state) => state.pager);
  const setState = useListStore((state) => state.setState);

  const [loading, setLoading] = useState(false);

  const fetchNodeListData = useMemoizedFn(async () => {
    const params: DescribeNodeListRequestParams = {
      page: pager?.page,
      pageSize: pager?.pageSize,
    };

    if (filters?.keyword) {
      params._name = filters.keyword;
    }

    const res = await DescribeNodeList(params);
    return res;
  });

  const fetchNodeCategories = useMemoizedFn(async (ids: number[]) => {
    const params: DescribeMultiNodesCategoriesRequestParams = {
      nids: ids.join(','),
    };
    const res = await DescribeMultiNodesCategories(params);
    return res;
  });

  const fetchListPageData = useMemoizedFn(async () => {
    setLoading(true);
    const nodeRes = await fetchNodeListData();
    const categoriesRes = await fetchNodeCategories(nodeRes?.nodes?.map(item => item?.nid || 0) || []);

    setState({
      nodeRes,
      categoryRes: categoriesRes,
      pager: {
        ...pager,
        total: nodeRes?.total || 0,
      }
    });
    setLoading(false);
  });

  const refresh = useMemoizedFn(() => {
    fetchListPageData();
  });

  return {
    loading,
    refresh,
  };
}
