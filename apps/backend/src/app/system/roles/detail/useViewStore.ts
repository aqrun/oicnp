'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { RoleModel } from '@/services';

export interface BaseViewState {
  visible: boolean;
  roleId?: number;
  role: RoleModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  roleId: 0,
  role: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
