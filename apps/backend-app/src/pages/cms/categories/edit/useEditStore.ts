'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import { CategoryModel } from "@repo/apis";

export interface BaseEditState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  catId: number;
  category: CategoryModel | undefined;
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
  visible: false,
  catId: 0,
  category: undefined,
  contentType: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));