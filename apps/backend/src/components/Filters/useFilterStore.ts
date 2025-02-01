'use client';

import { create } from 'zustand';
import { BaseState } from '@/stores/types';
import { FilterValues } from '@/types';

export type FilterState = BaseState<FilterValues> & {
  /**
   * 筛选数据
   */
  values: FilterValues,
  /**
   * 更新筛选数据
   */
  setValues: (payload: FilterValues) => void,
};

/**
 *  筛选数据
 */
export const useFilterStore = create<FilterState>()((set) => ({
  values: {},
  setValues: (payload) => set((state) => {
    return {
      ...state,
      values: {
        ...(state.values || {}),
        ...(payload || {}),
      },
    };
  }),
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
