import { useState } from 'react';
import { useMemoizedFn } from 'ahooks';
import { Input } from 'antd';
import { CLASS_PREFIX } from '~/constants';
import { useFilterStore } from '~/stores';
import { useDebounceFn } from 'ahooks';
import { FilterValues } from '~/types';
import { ChangeEvent } from 'react';

const { Search } = Input;

export interface SearchBoxProps {
  placeholder?: string;
  onSearch?: (value: FilterValues) => void;
  onChange?: (values: FilterValues, trigger?: string) => void;
}

export function SearchBox({
  placeholder,
  onSearch,
  onChange,
}: SearchBoxProps): JSX.Element {
  const values = useFilterStore((state) => state.values);
  const setFilterValues = useFilterStore((state) => state.setValues);
  const [value, setValue] = useState<string | undefined>(undefined);

  const {
    run: updateStore,
  } = useDebounceFn((newValues: FilterValues) => {
    setFilterValues(newValues);

    if (typeof onChange === 'function') {
      onChange(newValues, 'keyword');
    }
  }, { wait: 300 });

  const handleChange = useMemoizedFn((e: ChangeEvent<HTMLInputElement>) => {
    const newValue = e?.target?.value;
    setValue(newValue);
    
    const newValues: FilterValues = {
      ...values,
      keyword: newValue,
    };
    updateStore(newValues);
  });

  const handleSearch = useMemoizedFn((value: string) => {
    const newValues: FilterValues = {
      ...values,
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
