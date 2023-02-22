import { all, fork } from 'redux-saga/effects';
import { counterWatcher } from './counter/saga';

export function* sagas() {
  yield all([
    fork(counterWatcher),
  ]);
}
