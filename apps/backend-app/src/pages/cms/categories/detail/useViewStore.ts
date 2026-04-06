'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import { CategoryModel } from "@repo/apis";

export interface BaseViewState {
  visible: boolean;
  catId?: number;
  category: CategoryModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  catId: 0,
  category: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));