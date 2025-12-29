'use client';

import { create } from 'zustand';
import {
  DescribePoetryListWithChaptersResponseData,
} from '@repo/apis/client';

export interface BookState {
  loading?: boolean;
  hasMore?: boolean;
  poetryResList: DescribePoetryListWithChaptersResponseData[];
  pager: {
    page: number;
    pageSize: number;
    total: number;
  };
}

export const defaultState: BookState = {
  loading: false,
  hasMore: true,
  poetryResList: [],
  pager: {
    page: 1,
    pageSize: 10,
    total: 0,
  },
};

/**
 * 书籍状态数据
 */
export const useBookStore = create<BookState>()(() => ({
  ...defaultState,
}));
