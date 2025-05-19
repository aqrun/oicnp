'use client';

import { create } from 'zustand';
import type { BaseState } from '@/stores/types';
import { NoteModel } from '@/services';

export interface BaseEditState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  noteId: number;
  note: NoteModel | undefined;
};

export type EditState = BaseEditState & BaseState<BaseEditState>;

/**
 *  创建数据
 */
export const useEditStore = create<EditState>()((set) => ({
  visible: false,
  noteId: 0,
  note: undefined,
  contentType: '',
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
