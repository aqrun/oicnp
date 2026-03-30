'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import {
  DescribeLoginLogList,
} from './client';
import {
  DescribeLoginLogListRequestParams,
} from './types';

/**
 * 获取职位列表
 */
export function useFetchLoginLogList() {
  const [loading, setLoading] = useState(false);

  const fetchLoginLogList = useMemoizedFn(async (params: DescribeLoginLogListRequestParams) => {
    setLoading(true);
    const res = await DescribeLoginLogList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
    });
    setLoading(false);
    return res;
  })

  return {
    loading,
    fetchLoginLogList,
  };
}
