'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';

export interface BaseCreateState {
  visible?: boolean;
};

export type CreateState = BaseCreateState & BaseState<BaseCreateState>;

/**
 *  创建数据
 */
export const useCreateStore = create<CreateState>()((set) => ({
  visible: false,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
