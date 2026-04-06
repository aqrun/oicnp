'use client';

import { useMemoizedFn } from 'ahooks';
import {
  DescribeTagListRequestParams
} from './types';
import {
  DescribeTagList,
} from './client';

export function useFetchTags() {
  const fetchTags = useMemoizedFn(async (params: DescribeTagListRequestParams) => {
    const res = await DescribeTagList(params);
    return res;
  });

  return {
    fetchTags,
  };
}