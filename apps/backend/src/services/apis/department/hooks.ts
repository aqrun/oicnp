'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import {
  DescribeDepartmentList,
} from './client';
import {
  DescribeDepartmentListRequestParams,
} from './types';

/**
 * 获取部门列表
 */
export function useFetchDepartmentList() {
  const [loading, setLoading] = useState(false);

  const fetchDepartmentList = useMemoizedFn(async (params: DescribeDepartmentListRequestParams) => {
    setLoading(true);
    const res = await DescribeDepartmentList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
    });
    setLoading(false);
    return res;
  })

  return {
    loading,
    fetchDepartmentList,
  };
}
