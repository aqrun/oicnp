'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { NodeModel } from '@/services';

export interface BaseViewState {
  visible: boolean;
  nid?: number;
  node: NodeModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  nid: 0,
  node: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
