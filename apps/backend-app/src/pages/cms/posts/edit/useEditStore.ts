'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import { CategoryModel, NodeModel, TagModel } from "@repo/apis";

export interface BaseEditState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  nid: number;
  node: NodeModel | undefined;
  tags: TagModel[];
  /**
   * 当前选中的分类
   */
  categories: CategoryModel[];
  /**
   * 全部分类数据
   */
  categoryList: CategoryModel[];
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
  visible: false,
  nid: 0,
  node: undefined,
  tags: [],
  categories: [],
  categoryList: [],
  contentType: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));