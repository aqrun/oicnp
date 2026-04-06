'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import { UploadFileRes } from "@repo/apis";

export interface BaseEditState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  fileId: number;
  file: UploadFileRes | undefined;
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
  visible: false,
  fileId: 0,
  file: undefined,
  contentType: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));