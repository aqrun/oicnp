'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { TagModel } from '@/services';

export interface BaseTagsState {
  tags?: TagModel[];
  values?: string[];
};

export type TagsState = BaseTagsState & BaseState<BaseTagsState>;

/**
 *  标签数据
 */
export const useTagsStore = create<TagsState>()((set) => ({
  values: [],
  tags: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
