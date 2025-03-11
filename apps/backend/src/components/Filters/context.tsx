import { createContext, useContext } from 'react';
import { FilterValues } from '@/types';
import {
  FiltersProps,
} from './types';

export interface FilterScopeState extends FiltersProps {
  values: FilterValues;
  setValues: (payload: FilterValues) => FilterValues;
}

/**
 * 默认初始化数据
 */
const initialState: FilterScopeState = {
  values: {},
  setValues: () => ({}),
};

export const FilterContext = createContext<FilterScopeState>(initialState);

export function useFilterState() {
  const data = useContext(FilterContext);
  return data;
}
