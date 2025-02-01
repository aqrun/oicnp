'use client';

import { create } from 'zustand';
import { BaseState } from './types';

export interface BaseAppState {
  /**
   * 全局接口加载状态
   */
  loading: boolean;
}

export type AppState = BaseAppState & BaseState<BaseAppState>;

/**
 * 应用主状态数据
 */
export const useAppStore = create<AppState>()((set) => ({
  loading: false,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
