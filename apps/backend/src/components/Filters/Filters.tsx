'use client';

import { CLASS_PREFIX } from '@/constants';
import cls from 'clsx';
import { CreateButton } from './CreateButton';
import { RefreshButton } from './RefreshButton';
import { SearchBox } from './SearchBox';
import { EnumFilterTrigger, FilterValues } from '@/types';
import { Container } from './index.styled';

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
}

/**
 * 筛选组件
 */
export function Filters({
  createLabel,
  placeholder,
  onSearch,
  onChange,
  onCreate,
  onRefresh,
}: FiltersProps): JSX.Element {
  return (
    <Container>
      <div className={cls(`${CLASS_PREFIX}-filter-left`)}>
        {Boolean(onCreate) && (
          <CreateButton
            label={createLabel}
          />
        )}
        {Boolean(onSearch) && (
          <SearchBox
            placeholder={placeholder}
            onSearch={onSearch}
            onChange={onChange}
          />
        )}
      </div>
      <div className={cls(`${CLASS_PREFIX}-filter-right`)}>
        {Boolean(onRefresh) && (
          <RefreshButton
            onRefresh={onRefresh}
          />
        )}
      </div>
    </Container>
  );
}
