import { counterSlice } from './counterSlice';

export const getReducers = () => {
  return {
    counter: counterSlice.reducer,
  };
};