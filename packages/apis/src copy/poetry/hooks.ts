'use client';

import { useMemoizedFn } from 'ahooks';
import {
  DescribePoetryListWithChaptersRequestParams,
} from './types';
import {
  DescribePoetryListWithChapters,
} from './client';

export function useFetchPoetryListWithChapters() {
  const fetchPoetryListWithChapters = useMemoizedFn(async (params: DescribePoetryListWithChaptersRequestParams) => {
    const res = await DescribePoetryListWithChapters(params);
    return res;
  });

  return {
    fetchPoetryListWithChapters,
  };
}