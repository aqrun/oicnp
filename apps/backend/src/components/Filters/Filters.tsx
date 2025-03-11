'use client';

import { useState } from 'react';
import { CLASS_PREFIX } from '@/constants';
import cls from 'clsx';
import { CreateButton } from './CreateButton';
import { RefreshButton } from './RefreshButton';
import { SearchBox } from './SearchBox';
import { FilterValues } from '@/types';
import {
  FilterContext,
  FilterScopeState,
  useFilterState,
} from './context';
import {
  FiltersProps,
} from './types';
import { Container } from './index.styled';
import { useMemoizedFn } from 'ahooks';

/**
 * 筛选组件
 */
export function FiltersWidget(): JSX.Element {
  const {
    createLabel,
    onSearch,
    onCreate,
    onRefresh,
  } = useFilterState();

  return (
    <Container>
      <div className={cls(`${CLASS_PREFIX}-filter-left`)}>
        {Boolean(onCreate) && (
          <CreateButton
            label={createLabel}
            onCreate={onCreate}
          />
        )}
        {Boolean(onSearch) && (
          <SearchBox />
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

export function Filters(props: FiltersProps) {
  const [values, setValues] = useState<FilterValues>({});

  const updateValues = useMemoizedFn((payload: FilterValues = {}) => {
    const newState: FilterValues = {
      ...values,
      ...payload,
    };
    setValues(newState);
    return newState;
  });
  
  // context 数据
  const scopeData: FilterScopeState = {
    ...(props || {}),
    values,
    setValues: updateValues,
  };

  return (
    <FilterContext.Provider
      value={scopeData}
    >
      <FiltersWidget />
    </FilterContext.Provider>
  );
}
