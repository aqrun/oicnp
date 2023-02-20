import { all } from 'redux-saga/effects';
import { counterWatcher } from './counter';

export function* sagas() {
  yield all([
    counterWatcher(),
  ]);
}