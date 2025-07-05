'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import {
  FileModel,
} from '@/services';

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
