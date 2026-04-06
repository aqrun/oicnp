'use client';

import { useState } from 'react';
import type { DescribeNodeListRequestParams } from "@repo/apis";
import { nodeApis } from "#src/api";
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

    const res = await nodeApis.DescribeNodeList(params);
    return res;
  });

  const fetchListPageData = useMemoizedFn(async () => {
    setLoading(true);
    const nodeRes = await fetchNodeListData();
    
    setState({
      nodeRes,
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