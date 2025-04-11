'use client';

import { create } from 'zustand';
import type { BaseState, BaseListState } from '@/stores/types';

export type BaseMenuState = BaseListState & {
  refreshToken: string;
  _name?: string;
};

export type MenuState = BaseMenuState & BaseState<BaseMenuState>;

/**
 *  筛选数据
 */
export const useMenuStore = create<MenuState>()((set) => ({
  pager: {
    page: 1,
    pageSize: 1000,
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
