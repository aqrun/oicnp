'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import { TagModel } from "@repo/apis";

export interface BaseViewState {
  visible: boolean;
  tagId?: number;
  tag: TagModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  tagId: 0,
  tag: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));