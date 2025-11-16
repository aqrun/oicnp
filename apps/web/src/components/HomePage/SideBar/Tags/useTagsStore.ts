'use client';

import { create } from 'zustand';
import {
  DescribeTagListResponseData,
} from '@repo/apis/client';

export interface TagsState {
  loading?: boolean;
  tagsRes?: DescribeTagListResponseData;
}

/**
 * 应用主状态数据
 */
export const useTagsStore = create<TagsState>()((set) => ({
  loading: false,
  tagsRes: undefined,
}));
