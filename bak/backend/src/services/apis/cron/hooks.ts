'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import {
  DescribeCronList,
} from './client';
import {
  DescribeCronListRequestParams,
} from './types';

/**
 * 获取定时任务列表
 */
export function useFetchCronList() {
  const [loading, setLoading] = useState(false);

  const fetchCronList = useMemoizedFn(async (params: DescribeCronListRequestParams = {}) => {
    setLoading(true);
    const res = await DescribeCronList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
    });
    setLoading(false);
    return res;
  })

  return {
    loading,
    fetchCronList,
  };
}
