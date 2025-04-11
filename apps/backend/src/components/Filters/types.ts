'use client';

import {
  FilterValues,
  EnumFilterTrigger,
} from '@/types';

export interface FiltersProps {
  /**
   * 创建按钮显示
   */
  createLabel?: string;
  /**
   * 搜索内容提示
   */
  placeholder?: string;
  /**
   * 搜索事件
   */
  onSearch?: (values: FilterValues) => void;
  /**
   * 筛选值变化事件
   */
  onChange?: (values: FilterValues, trigger?: EnumFilterTrigger) => void;
  /**
   * 点击创建
   */
  onCreate?: () => void;
  /**
   * 点击刷新
   */
  onRefresh?: () => void;
  /**
   * 展开收起
   * @returns
   */
  onExpand?: () => void;
}