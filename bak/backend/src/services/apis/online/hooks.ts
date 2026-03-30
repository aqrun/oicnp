'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import {
  DescribeOnlineList,
} from './client';
import {
  DescribeOnlineListRequestParams,
} from './types';

/**
 * 获取文件列表
 */
export function useFetchOnlineList() {
  const [loading, setLoading] = useState(false);

  const fetchOnlineList = useMemoizedFn(async (params: DescribeOnlineListRequestParams = {}) => {
    setLoading(true);
    const res = await DescribeOnlineList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
    });
    setLoading(false);
    return res;
  })

  return {
    loading,
    fetchOnlineList,
  };
}
