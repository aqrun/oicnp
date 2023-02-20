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

  return store;
};

export type RootState = ReturnType<typeof createStore>;

export const wrapper = createWrapper(createStore);