
export interface BaseState<T> {
  setState: (payload: Partial<T>) => void;
}
