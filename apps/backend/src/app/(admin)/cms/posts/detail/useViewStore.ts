'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import {
  NodeModel,
  NodeBody,
  TagModel,
  CategoryModel,
} from '@/services';

export interface BaseViewState {
  visible: boolean;
  nid?: number;
  node?: NodeModel;
  body?: NodeBody;
  tags: Array<TagModel>;
  categories: Array<CategoryModel>;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  nid: 0,
  node: undefined,
  body: undefined,
  tags: [],
  categories: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
