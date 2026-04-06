'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import { OperationLogModel } from "@repo/apis";

export interface BaseViewState {
  visible: boolean;
  operationLogId?: number;
  operationLog: OperationLogModel | undefined;
};

export type ViewState = BaseViewState & BaseState<BaseViewState>;

/**
 *  查看数据
 */
export const useViewStore = create<ViewState>()((set) => ({
  visible: false,
  operationLogId: 0,
  operationLog: undefined,
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));