'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import { NoteModel } from "@repo/apis";

export interface BaseViewState {
  visible: boolean;
  noteId?: number;
  note: NoteModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  noteId: 0,
  note: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));