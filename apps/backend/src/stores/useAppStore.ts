'use client';

import { create } from 'zustand';
import { BaseState } from './types';
import {
  FailModel,
} from '@/types';
import {
  UserModel,
} from '@/services';

export interface BaseAppState {
  /**
   * 全局接口加载状态
   */
  loading: boolean;
  initComplete: boolean;
  errors: Array<FailModel>;
  addError: (failModel: FailModel) => void,
  /**
   * 当前登陆用户信息
   */
  user?: UserModel;
  updateToken?: string;
}

export type AppState = BaseAppState & BaseState<BaseAppState>;

/**
 * 应用主状态数据
 */
export const useAppStore = create<AppState>()((set) => ({
  user: undefined,
  loading: false,
  initComplete: false,
  errors: [],
  updateToken: '',
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
