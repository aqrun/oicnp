import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import {
  categoryApis,
} from '#src/api';
import {
  DescribeCategoryListRequestParams,
} from '@repo/apis';

/**
 * 获取分类列表
 */
export function useFetchCategoryList() {
  const [loading, setLoading] = useState(false);

  const fetchCategoryList = useMemoizedFn(async (params: DescribeCategoryListRequestParams) => {
    setLoading(true);
    const res = await categoryApis.DescribeCategoryList({
      page: params.page || 1,
      pageSize: params.pageSize || 10,
    });
    setLoading(false);
    return res;
  })

  return {
    loading,
    fetchCategoryList,
  };
}
