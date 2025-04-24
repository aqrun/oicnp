'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { RoleModel, UserModel } from '@/services';

export interface BaseEditState {
  user: UserModel | undefined;
  userRoles: Array<RoleModel>;
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
  user: undefined,
  userRoles: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
