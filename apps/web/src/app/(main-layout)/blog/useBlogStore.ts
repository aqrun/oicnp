'use client';

import { create } from 'zustand';
import {
  DescribeNodeListResponseData,
} from '@repo/apis/client';

export interface BlogState {
  loading?: boolean;
  hasMore?: boolean;
  nodeResList?: DescribeNodeListResponseData[];
  pager: {
    page: number;
    pageSize: number;
    total: number;
  };
}

/**
 * 应用主状态数据
 */
export const useBlogStore = create<BlogState>()(() => ({
  loading: false,
  hasMore: true,
  pager: {
    page: 1,
    pageSize: 10,
    total: 0,
  },
  nodes: [],
}));
