'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import type { PermissionModel } from '@repo/apis';

export interface BaseViewState {
  visible: boolean;
  permissionId?: number;
  permission: PermissionModel | undefined;
  parentPermission: PermissionModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  permissionId: 0,
  permission: undefined,
  parentPermission: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
