import { configureStore } from '@reduxjs/toolkit';
import createSagaMiddleware from 'redux-saga';
import { createWrapper } from 'next-redux-wrapper';

import { getReducers } from './reducers';
import { sagas } from './sagas';

export const createStore = () => {
  const sagaMiddleware = createSagaMiddleware();
  const store = configureStore({
    reducer: getReducers(),
    middleware: (getDefaultMiddleware) => {
      return getDefaultMiddleware().concat(sagaMiddleware);
    },
    devTools: true,
  });

  // run saga
  sagaMiddleware.run(sagas);
  type a = ReturnType<typeof store.getState>;
  return store;
};

type RootStore = ReturnType<typeof createStore>;
export type RootState = ReturnType<RootStore['getState']>;

export const wrapper = createWrapper(createStore);