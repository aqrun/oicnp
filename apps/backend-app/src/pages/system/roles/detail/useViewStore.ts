'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import type { RoleModel, PermissionModel } from '@repo/apis';

export interface BaseViewState {
  visible: boolean;
  roleId?: number;
  role: RoleModel | undefined;
  rolePermissions: Array<PermissionModel>;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  roleId: 0,
  role: undefined,
  rolePermissions: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
