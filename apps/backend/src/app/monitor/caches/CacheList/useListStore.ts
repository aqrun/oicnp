'use client';

import { create } from 'zustand';
import type { BaseState, BaseListState } from '@/stores/types';
import {
  DescribeCacheListResponseData,
  DescribeCacheScopeListResponseData,
  DescribeCacheDetailResponseData,
} from '@/services';

export type BaseListPageState = BaseListState & {
  cachesRes: DescribeCacheListResponseData | undefined;
  scopesRes: DescribeCacheScopeListResponseData | undefined;
  cacheDetailRes: DescribeCacheDetailResponseData | undefined;
  /**
   * 当前选中的分类
   */
  scope: string;
  cacheKey: string;
  refreshToken: string;
  cacheRefreshToken: string;
  _name?: string;
};

export type ListState = BaseListPageState & BaseState<BaseListPageState>;

/**
 *  筛选数据
 */
export const useListStore = create<ListState>()((set) => ({
  cachesRes: undefined,
  scopesRes: undefined,
  cacheDetailRes: undefined,
  scope: '',
  cacheKey: '',
  pager: {
    page: 1,
    pageSize: 10,
    total: 0,
  },
  filters: {
    keyword: '',
  },
  refreshToken: '',
  cacheRefreshToken: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
