'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { TagModel } from '@/services';

export interface BaseEditState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  tagId: number;
  tag: TagModel | undefined;
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
  visible: false,
  tagId: 0,
  tag: undefined,
  contentType: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
