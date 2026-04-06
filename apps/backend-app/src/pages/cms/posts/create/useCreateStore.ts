'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import { CategoryModel } from "@repo/apis";

export interface BaseCreateState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  tags?: string[];
  categories?: CategoryModel[];
};

export type CreateState = BaseCreateState & BaseState<BaseCreateState>;

/**
 *  创建数据
 */
export const useCreateStore = create<CreateState>()((set) => ({
  visible: false,
  contentType: '',
  tags: [],
  categories: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));