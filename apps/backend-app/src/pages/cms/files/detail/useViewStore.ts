'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import {   FileModel } from "@repo/apis";

export interface BaseViewState {
  visible: boolean;
  fileId?: number;
  file?: FileModel;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  fileId: 0,
  file: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));