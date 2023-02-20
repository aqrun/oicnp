import { createSlice } from '@reduxjs/toolkit';

const initialState = {
  count: 0,
};

export const counterSlice = createSlice({
  name: 'counter',
  initialState,
  reducers: {
    increment: (state) => {
      state.count += 1;
    },
    incrementWith: (state, action) => {
      console.log('inwith', action);
      state.count += action?.payload || 0;
    }
  },
});

export const {
  increment,
  incrementWith,
} = counterSlice.actions;
export const selectCounter = (state) => state.counter;
export const selectCount = (state) => state.counter.count;
