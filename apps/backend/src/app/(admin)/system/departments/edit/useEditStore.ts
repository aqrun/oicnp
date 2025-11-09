'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { DepartmentModel } from '@/services';

export interface BaseEditState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  departmentId: number;
  department: DepartmentModel | undefined;
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
  visible: false,
  departmentId: 0,
  department: undefined,
  contentType: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
