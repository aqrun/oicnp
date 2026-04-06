'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import type { PositionModel } from '@repo/apis';

export interface BaseViewState {
  visible: boolean;
  positionId?: number;
  position: PositionModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  positionId: 0,
  position: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
