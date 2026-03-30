'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { DepartmentModel } from '@/services';

export interface BaseViewState {
  visible: boolean;
  departmentId?: number;
  department: DepartmentModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  departmentId: 0,
  department: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
