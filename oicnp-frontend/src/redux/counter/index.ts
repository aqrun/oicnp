import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { RootState } from '../store';
import {
  CounterState,
  IncrementAsyncAction,
  IncrementAsyncPayload,
  IncrementAsyncMeta,
} from './types';

const initialState: CounterState = {
  count: 0,
  isLoading: false,
};

export const counterSlice = createSlice({
  name: 'counter',
  initialState,
  reducers: {
    increment: (state) => {
      state.count += 1;
    },
    incrementAsync: (state, action: PayloadAction<IncrementAsyncPayload>) => {
      console.log('inwith', action);
      state.isLoading = true;
    },
    incrementAsyncSucceed: (state, action: PayloadAction<number>) => {
      state.count += action?.payload;
      state.isLoading = false;
    },
    incrementAsyncFailed: (state, action) => {
      state.isLoading = false;
    },
    setState: (state, action: PayloadAction<Partial<CounterState>>) => {
      Object.keys(action?.payload || {})?.forEach((item) => {
        (state as any)[item] = (action?.payload as any)?.[item];
      });
    }
  },
});

export const {
  increment,
  incrementAsync,
  incrementAsyncSucceed,
  incrementAsyncFailed,
} = counterSlice.actions;

export const selectCounter = (state: RootState) => state.counter;
export const selectCount = (state: RootState) => state.counter.count;
