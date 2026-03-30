'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import {
  DescribeOperationLogList,
} from './client';
import {
  DescribeOperationLogListRequestParams,
} from './types';

/**
 * 获取职位列表
 */
export function useFetchOperationLogList() {
  const [loading, setLoading] = useState(false);

  const fetchOperationLogList = useMemoizedFn(async (params: DescribeOperationLogListRequestParams) => {
    setLoading(true);
    const res = await DescribeOperationLogList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
    });
    setLoading(false);
    return res;
  })

  return {
    loading,
    fetchOperationLogList,
  };
}
