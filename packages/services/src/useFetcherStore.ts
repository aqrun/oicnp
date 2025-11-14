'use client';

import { create } from 'zustand';
import {
  FailModel,
} from './types';

export interface BaseFetcherState {
  /**
   * 全局接口加载状态
   */
  loading: boolean;
  errors: Array<FailModel>;
  addError: (failModel: FailModel) => void,
}

export type FetcherState = BaseFetcherState;

/**
 * 应用主状态数据
 */
export const useFetcherStore = create<FetcherState>()((set) => ({
  loading: false,
  errors: [],
  addError: (failModel: FailModel) => set((state) => {
    const newErrors = [
      ...state.errors,
      failModel,
    ];
    return {
      ...state,
      errors: newErrors,
    };
  }),
}));
