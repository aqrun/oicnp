'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import {
  DescribeTagList,
} from './client';
import {
  DescribeTagListRequestParams,
} from './types';

/**
 * 获取标签列表
 */
export function useFetchTagList() {
  const [loading, setLoading] = useState(false);

  const fetchTagList = useMemoizedFn(async (params: DescribeTagListRequestParams) => {
    setLoading(true);
    const res = await DescribeTagList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
    });
    setLoading(false);
    return res;
  })

  return {
    loading,
    fetchTagList,
  };
}
