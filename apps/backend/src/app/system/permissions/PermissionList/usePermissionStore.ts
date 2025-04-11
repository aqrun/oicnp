'use client';

import { create } from 'zustand';
import type { BaseState, BaseListState } from '@/stores/types';

export type BasePermissionState = BaseListState & {
  refreshToken: string;
  _name?: string;
};

export type PermissionState = BasePermissionState & BaseState<BasePermissionState>;

/**
 *  筛选数据
 */
export const usePermissionStore = create<PermissionState>()((set) => ({
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
