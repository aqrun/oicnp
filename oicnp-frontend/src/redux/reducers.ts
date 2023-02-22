import { counterSlice } from './counter';

export const getReducers = () => {
  return {
    counter: counterSlice.reducer,
  };
};
