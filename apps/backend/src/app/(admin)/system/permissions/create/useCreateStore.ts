'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';

export interface BaseCreateState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  /**
   * 初始化父级权限
   */
  initPid?: number;
};

export type CreateState = BaseCreateState & BaseState<BaseCreateState>;

/**
 *  创建数据
 */
export const useCreateStore = create<CreateState>()((set) => ({
  visible: false,
  contentType: '',
  initPid: 0,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
