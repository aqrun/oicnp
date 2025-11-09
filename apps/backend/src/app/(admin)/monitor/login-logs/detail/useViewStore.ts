'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { LoginLogModel } from '@/services';

export interface BaseViewState {
  visible: boolean;
  loginLogId?: number;
  loginLog: LoginLogModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  loginLogId: 0,
  loginLog: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
