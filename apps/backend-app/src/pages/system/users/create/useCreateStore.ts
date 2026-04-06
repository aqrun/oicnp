'use client';

import { create } from 'zustand';
import type { BaseState } from '#src/store/types';
import type { RoleModel } from '@repo/apis';

export interface BaseCreateState {
  visible?: boolean;
  /**
   * 内容类型
   */
  contentType?: string;
  roleList?: Array<RoleModel>;
};

export type CreateState = BaseCreateState & BaseState<BaseCreateState>;

/**
 *  创建数据
 */
export const useCreateStore = create<CreateState>()((set) => ({
  visible: false,
  contentType: '',
  roleList: [],
  setState: (payload) => set((state) => {
    return {
      ...state,
      ...(payload || {}),
    };
  }),
}));
