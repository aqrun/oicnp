'use client';

import { create } from 'zustand';
import { BaseState } from './types';
import {
  FailModel,
} from '@/types';
import {
  DescribeUserResponseData,
} from '@/services/types';

export interface BaseAppState {
  /**
   * 全局接口加载状态
   */
  loading: boolean;
  errors: Array<FailModel>;
  addError: (failModel: FailModel) => void,
  /**
   * 当前登陆用户信息
   */
  user?: DescribeUserResponseData;
}

export type AppState = BaseAppState & BaseState<BaseAppState>;

/**
 * 应用主状态数据
 */
export const useAppStore = create<AppState>()((set) => ({
  user: undefined,
  loading: false,
  errors: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
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
