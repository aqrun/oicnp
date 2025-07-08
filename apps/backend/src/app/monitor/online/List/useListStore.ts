'use client';

import { create } from 'zustand';
import type { BaseState, BaseListState } from '@/stores/types';
import { OnlineModel } from '@/services';

export type BaseListPageState = BaseListState & {
  listRes: OnlineModel[];
  refreshToken: string;
  _name?: string;
};

export type ListState = BaseListPageState & BaseState<BaseListPageState>;

/**
 *  筛选数据
 */
export const useListStore = create<ListState>()((set) => ({
  listRes: [],
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
