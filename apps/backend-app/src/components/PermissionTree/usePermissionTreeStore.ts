'use client';

import { create } from 'zustand';
import { BaseState } from '#src/store/types';
import type { PermissionTreeItem } from '@repo/apis';

export interface BaseTreeState {
  treeData: PermissionTreeItem[] | undefined;
  loading: boolean;
  checkedKeys: Array<React.Key> | undefined;
}

export type TreeState = BaseTreeState & BaseState<BaseTreeState>;

/**
 * 权限树状态数据
 */
export const usePermissionTreeStore = create<TreeState>()((set) => ({
  loading: false,
  treeData: undefined,
  checkedKeys: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
