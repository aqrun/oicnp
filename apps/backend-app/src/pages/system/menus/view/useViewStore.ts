'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import type { MenuModel, PermissionModel } from '@repo/apis';

export interface BaseViewState {
  visible: boolean;
  menuId?: number;
  menu: MenuModel | undefined;
  menuPermissions: Array<PermissionModel>;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  menuId: 0,
  menu: undefined,
  menuPermissions: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
