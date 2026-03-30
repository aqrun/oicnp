'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { PermissionModel } from '@/services';

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
