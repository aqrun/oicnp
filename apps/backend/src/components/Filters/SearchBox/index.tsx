'use client';

import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Input } from 'antd';
import { CLASS_PREFIX } from '@/constants';
import { useDebounceFn } from 'ahooks';
import { FilterValues, EnumFilterTrigger } from '@/types';
import { ChangeEvent } from 'react';
import { useFilterState } from '../context';

const { Search } = Input;

/**
 * 关键词搜索框
 */
export function SearchBox(): JSX.Element {
  const {
    setValues,
    placeholder,
    onSearch,
    onChange,
  } = useFilterState();
  const [value, setValue] = useState<string | undefined>(undefined);

  /**
   * 输入变化延时处理
   */
  const {
    run: updateStore,
  } = useDebounceFn((newValues: FilterValues) => {
    setValues(newValues);

    if (typeof onChange === 'function') {
      onChange(newValues, EnumFilterTrigger.keyword);
    }
  }, { wait: 300 });
  
  /**
   * 输入输入变化事件
   */
  const handleChange = useMemoizedFn((e: ChangeEvent<HTMLInputElement>) => {
    const newValue = e?.target?.value;
    setValue(newValue);
    
    const newValues: FilterValues = {
      // ...values,
      keyword: newValue,
    };
    updateStore(newValues);
  });

  /**
   * 点击搜索
   */
  const handleSearch = useMemoizedFn((value: string) => {
    const newValues: FilterValues = {
      // ...values,
      keyword: value,
    };

    if (typeof onSearch === 'function') {
      onSearch(newValues);
    }
  });

  return (
    <Search
      placeholder={placeholder || '请输入关键词'}
      allowClear
      onSearch={handleSearch}
      onChange={handleChange}
      className={`${CLASS_PREFIX}-filter-search-box`}
      value={value}
    />
  );
}
