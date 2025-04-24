'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import {
  RoleModel,
} from '@/services';

export interface BaseCreateState {
  roleList?: Array<RoleModel>;
};

export type CreateState = BaseCreateState & BaseState<BaseCreateState>;

/**
 *  创建数据
 */
export const useCreateStore = create<CreateState>()((set) => ({
  roleList: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
