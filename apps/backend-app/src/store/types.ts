import { FilterValues } from '#src/types/app';

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
