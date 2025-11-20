'use client';

import { create } from 'zustand';

export interface BookState {
  title?: string;
  category?: string;
}

/**
 * 书籍状态数据
 */
export const useBookStore = create<BookState>()((set) => ({
  title: '',
  category: 'all',
}));
