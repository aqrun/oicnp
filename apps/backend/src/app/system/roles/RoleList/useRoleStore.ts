'use client';

import { create } from 'zustand';
import type { BaseState, BaseListState } from '@/stores/types';

export type BaseRoleState = BaseListState & {
  refreshToken: string;
  _name?: string;
};

export type UserState = BaseRoleState & BaseState<BaseRoleState>;

/**
 *  筛选数据
 */
export const useRoleStore = create<UserState>()((set) => ({
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
