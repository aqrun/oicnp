'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import type { MenuModel, PermissionModel } from '@repo/apis';

export interface BaseEditState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  menuId: number;
  menu: MenuModel | undefined;
  menuPermissions: Array<PermissionModel>;
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
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
