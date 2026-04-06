'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import type { UserModel, RoleModel } from '@repo/apis';

export interface BaseEditState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  uid: number;
  user: UserModel | undefined;
  userRoles: Array<RoleModel>;
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
  visible: false,
  uid: 0,
  user: undefined,
  userRoles: [],
  contentType: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
