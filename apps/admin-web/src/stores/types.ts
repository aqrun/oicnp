import { FilterValues } from '~/types';

export interface BaseState<T> {
  setState: (payload: Partial<T>) => void;
}

export interface BaseListState {
  filters?: FilterValues;
  pager: {
    page: number;
    pageSize: number;
    total: number;
  };
}
