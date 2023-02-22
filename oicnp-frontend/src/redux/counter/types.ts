import { PayloadAction } from '@reduxjs/toolkit';

export interface CounterState {
  count: number;
  isLoading: boolean;
}

export interface IncrementAsyncPayload {
  num: number;
}

export interface IncrementAsyncMeta {
  actionResolved?: () => void;
}

export type IncrementAsyncAction = Partial<PayloadAction<IncrementAsyncPayload, string, IncrementAsyncMeta>>;

