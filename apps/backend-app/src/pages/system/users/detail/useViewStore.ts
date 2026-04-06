'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import type { UserModel } from '@repo/apis';

export interface BaseViewState {
  visible: boolean;
  uid?: number;
  user: UserModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  uid: 0,
  user: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
