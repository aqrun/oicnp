'use client';

import { create } from 'zustand';
import type { BaseState, BaseListState } from '@/stores/types';
import {
  DescribeNodeListResponseData,
} from '@/services';

export type BaseListPageState = BaseListState & {
  refreshToken: string;
  _name?: string;
  nodeRes: DescribeNodeListResponseData | undefined;
};

export type ListState = BaseListPageState & BaseState<BaseListPageState>;

/**
 *  筛选数据
 */
export const useListStore = create<ListState>()((set) => ({
  nodeRes: undefined,
  pager: {
    page: 1,
    pageSize: 10,
    total: 0,
  },
  filters: {
    keyword: '',
  },
  refreshToken: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
