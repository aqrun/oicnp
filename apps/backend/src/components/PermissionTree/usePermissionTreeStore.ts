'use client';

import { create } from 'zustand';
import { BaseState } from '@/stores/types';
import {
  PermissionTreeItem,
} from '@/services';

export interface BaseTreeState {
  treeData: PermissionTreeItem[];
  loading: boolean;
}

export type TreeState = BaseTreeState & BaseState<BaseTreeState>;

/**
 * 权限树状态数据
 */
export const usePermissionTreeStore = create<TreeState>()((set) => ({
  loading: false,
  treeData: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
